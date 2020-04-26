use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::game::graphical_segment::GraphicalSegment;
use crate::network::networking_message_types::*;
use crate::network::networking_segment::*;
use crate::network::networking_structs::*;
use crate::players::inputs::*;
use std::panic;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

use crate::game::logic::logic_segment::*;
use crate::game::logic::logic_head_sim_segment::*;
use crate::game::timekeeping::*;
use crate::game::logic::data_storage_manager::*;
use crate::game::logic::logic_data_storage::*;
use crate::game::client_input_handler_segment::*;
use crate::game::scheduler_segment::*;


struct Client{
    player_name: String,
    connection_ip: String
}

impl Client{
    fn init_networking(&self, connection_target_ip: &String) -> NetworkingSegmentEx {
        let mut net_seg_in = NetworkingSegmentIn::new(connection_target_ip.clone());
        let mut net_seg_ex = net_seg_in.start_net();
        return net_seg_ex;
    }
    fn init_data_store(&self, storage: LogicDataStorage) -> DataStorageManagerEx {
        let manager = DataStorageManagerIn::new(storage);
        return manager.init_data_storage();
    }
    fn init_tail_sim(&self, known_frame: KnownFrameInfo, tail_state: GameState, data_store: Arc<RwLock<LogicDataStorage>>) -> LogicSegmentTailerEx{
        let tail_logic_in = LogicSegmentTailerIn::new(known_frame, tail_state, data_store);
        return tail_logic_in.start_logic_tail();
    }
    fn init_head_sim(&self, known_frame: KnownFrameInfo, tail_state: Arc<RwLock<GameState>>, data_store: Arc<RwLock<LogicDataStorage>>)-> LogicHeadSimEx {
        let head_logic_in = LogicHeadSimIn::new(known_frame, tail_state, data_store);
        return head_logic_in.start();
    }
    fn init_net_rec_handling(&self, incoming_messages: Receiver<NetMessageType>, to_logic: Sender<LogicInwardsMessage>){
        thread::spawn(move || {
            loop{
                match incoming_messages.recv().unwrap(){
                    NetMessageType::GameUpdate(update) => {
                        if crate::SEND_DEBUG_MSGS{
                            println!("Net rec message: {:?}", update);
                        }
                        to_logic.send(update).unwrap();
                    },
                    NetMessageType::LocalCommand(_) => {panic!("Not implemented!")},
                    NetMessageType::PingTestResponse(_) => {
                        // Do nothing. Doesn't matter that intro stuff is still floating when we move on.
                    }
                    _ => {
                        panic!("Client shouldn't be getting a message of this type (or at this time)!")
                    }
                }
            }
        });
    }
    fn init_graphics(&self, state_to_render: Arc<RwLock<GameState>>, my_player_id: PlayerID) -> Receiver<InputChange>{
        let seg_graph = GraphicalSegment::new(state_to_render, my_player_id);
        return seg_graph.start();
    }
    fn start(self, test_arg: i64){
        let mut seg_net = self.init_networking(&self.connection_ip);
        let temp = SystemTime::now();
        let clock_offset_ns = seg_net.perform_ping_tests_get_clock_offset();
        println!("Clock offset: {}nanos or {}ms", clock_offset_ns, clock_offset_ns / 1_000_000);

        seg_net.send_greeting(&self.player_name);
        let welcome_info = seg_net.receive_welcome_message();

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
        let seg_logic_header = self.init_head_sim(synced_frame_info.clone(), seg_logic_tailer.tail_lock, seg_data_storage.clone_lock_ref());
        let seg_graphics = self.init_graphics(seg_logic_header.head_lock, welcome_info.assigned_player_id);


        let seg_input_dist = InputHandlerIn::new(seg_graphics, synced_frame_info.clone(),
                                                 welcome_info.assigned_player_id, welcome_info.you_initialize_frame);



        let my_to_net = seg_net.net_sink.clone();
        let my_to_data = seg_data_storage.logic_msgs_sink.clone();
        let frame_to_init_my_inputs = welcome_info.you_initialize_frame - HEAD_AHEAD_FRAME_COUNT;
        if crate::SEND_DEBUG_MSGS{
            println!("Frame to init my own inputs: {}", frame_to_init_my_inputs);
        }
        seg_scheduler.schedule_event(Box::new(move ||{
            seg_input_dist.start_dist(my_to_data, my_to_net);
        }), frame_to_init_my_inputs);

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





























