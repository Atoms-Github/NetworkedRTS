use std::panic;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, SystemTime};

use crate::client::connect_net_seg::*;
use crate::client::graphical_seg::*;
use crate::client::input_handler_seg::*;
use crate::client::logic_sim_header_seg::*;
use crate::common::gameplay::game::game_state::*;
use crate::common::logic::logic_sim_tailer_seg::*;
use crate::common::network::external_msg::*;
use crate::common::sim_data::input_state::*;
use crate::common::sim_data::sim_data_storage::*;
use crate::common::sim_data::sim_data_storage_manager::*;
use crate::common::time::scheduler_segment::*;
use crate::common::time::timekeeping::*;
use crate::common::types::*;

struct Client{
    player_name: String,
    connection_ip: String
}

impl Client{
    fn init_networking(&self, connection_target_ip: &String) -> ConnectNetEx {

        let mut net_seg_in = ConnectNetIn::new(connection_target_ip.clone());
        let mut net_seg_ex = net_seg_in.start_net();
        return net_seg_ex;
    }
    fn init_data_store(&self, storage: SimDataStorage) -> SimDataStorageManagerEx {
        let manager = SimDataStorageManagerIn::new(storage);
        return manager.init_data_storage();
    }
    fn init_tail_sim(&self, known_frame: KnownFrameInfo, tail_state: GameState, data_store: Arc<RwLock<SimDataStorage>>) -> LogicSimTailerEx{
        let tail_logic_in = LogicSegmentTailerIn::new(known_frame, tail_state, data_store);
        return tail_logic_in.start_logic_tail();
    }
    fn init_head_sim(&self, known_frame: KnownFrameInfo, tail_rec: Receiver<GameState>, data_store: Arc<RwLock<SimDataStorage>>)-> LogicSimHeaderEx {
        let head_logic_in = LogicSimHeaderIn::new(known_frame, tail_rec, data_store);
        return head_logic_in.start();
    }
    fn init_net_rec_handling(&self, incoming_messages: Receiver<ExternalMsg>, to_logic: Sender<LogicInwardsMessage>){
        thread::spawn(move || {
            loop{
                match incoming_messages.recv().unwrap(){
                    ExternalMsg::GameUpdate(update) => {
                        if crate::DEBUG_MSGS_MAIN {
                            println!("Net rec message: {:?}", update);
                        }
                        to_logic.send(update).unwrap();
                    },
                    ExternalMsg::LocalCommand(_) => {panic!("Not implemented!")},
                    ExternalMsg::PingTestResponse(_) => {
                        // Do nothing. Doesn't matter that intro stuff is still floating when we move on.
                    }
                    _ => {
                        panic!("Client shouldn't be getting a message of this type (or at this time)!")
                    }
                }
            }
        });
    }
    fn init_graphics(&self, render_states_rec: Receiver<GameState>, my_player_id: PlayerID) -> Receiver<InputChange>{
        let seg_graph = GraphicalSeg::new(render_states_rec, my_player_id);
        return seg_graph.start();
    }
    fn start(self, test_arg: i64){
        let mut seg_net = self.init_networking(&self.connection_ip);
        let temp = SystemTime::now();
        let clock_offset_ns = seg_net.perform_ping_tests_get_clock_offset();
        println!("Clock offset: {}nanos or {}ms", clock_offset_ns, clock_offset_ns / 1_000_000);

        seg_net.send_greeting(&self.player_name);
        let welcome_info = seg_net.receive_welcome_message();
        // Now that we've downloaded, then we can let everyone know in advance that we're coming as a new player.
        seg_net.send_init_me_msg(welcome_info.you_initialize_frame, welcome_info.assigned_player_id);

        let mut synced_frame_info = welcome_info.known_frame_info;
        println!("Before: {:?}", synced_frame_info);
        synced_frame_info.apply_offset(-clock_offset_ns); // Things work out that this is negative.
        // Known frame checks time between known and now.
        // If the server clock is fast, then we want to decrease our known one so we're using info from the future and vice versa.
        // Simpler explaination:
        // If server is fast, then we need to pull it back to convert it into local client time.
        println!("After: {:?}", synced_frame_info);

        let mut seg_scheduler = SchedulerSegIn::new(synced_frame_info.clone()).start();

        let seg_data_storage = self.init_data_store(welcome_info.frames_gathered_so_far);
        let seg_logic_tailer = self.init_tail_sim(synced_frame_info.clone(), welcome_info.game_state, seg_data_storage.clone_lock_ref());
        let seg_logic_header = self.init_head_sim(synced_frame_info.clone(), seg_logic_tailer.new_tail_states_rec, seg_data_storage.clone_lock_ref());
        let input_changes = self.init_graphics(seg_logic_header.head_rec, welcome_info.assigned_player_id);

        let seg_input_dist = InputHandlerIn::new(synced_frame_info, welcome_info.assigned_player_id,
                                                  seg_data_storage.logic_msgs_sink.clone(),
                                                 seg_net.net_sink.clone());



        if crate::DEBUG_MSGS_MAIN {
            println!("Frame to init my own sync: {}", welcome_info.you_initialize_frame);
        }
        seg_scheduler.schedule_event(Box::new(move ||{
            seg_input_dist.start_dist(input_changes);
        }), welcome_info.you_initialize_frame);

        self.init_net_rec_handling(seg_net.net_rec, seg_data_storage.logic_msgs_sink);


        loop{
            thread::sleep(Duration::from_millis(10000));
        }
    }
}




pub fn client_main(connection_target_ip: String, test_arg: i64){
    println!("Starting as client.");
    let client = Client{
        player_name: String::from("Atomserdiah"),
        connection_ip: connection_target_ip,
    };
    client.start(test_arg);
}





























