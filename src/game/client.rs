use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::game::graphical_segment::GraphicalSegment;
use crate::network::networking_message_types::*;
use crate::network::networking_segment::*;
use crate::network::networking_structs::*;
use crate::players::inputs::*;
use std::panic;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use crate::game::logic::logic_segment::*;
use crate::game::logic::logic_head_sim_segment::*;
use crate::game::synced_data_stream::*;
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
                        to_logic.send(update).unwrap();
                    },
                    NetMessageType::LocalCommand(_) => {panic!("Not implemented!")},
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
    fn start(self){
        let mut set_net = self.init_networking(&self.connection_ip);
        set_net.send_greeting(&self.player_name);
        let welcome_info = set_net.receive_welcome_message();

        let mut seg_scheduler = SchedulerSegIn::new(welcome_info.known_frame_info.clone()).start();

        let seg_data_storage = self.init_data_store(welcome_info.frames_gathered_so_far);
        let seg_logic_tailer = self.init_tail_sim(welcome_info.known_frame_info.clone(), welcome_info.game_state, seg_data_storage.clone_lock_ref());
        let seg_logic_header = self.init_head_sim(welcome_info.known_frame_info.clone(), seg_logic_tailer.tail_lock, seg_data_storage.clone_lock_ref());
        let seg_graphics = self.init_graphics(seg_logic_header.head_lock, welcome_info.assigned_player_id);


        let seg_input_dist = InputHandlerIn::new(seg_graphics, welcome_info.known_frame_info.clone(), welcome_info.assigned_player_id);



        let my_to_net = set_net.net_sink.clone();
        let my_to_data = seg_data_storage.logic_msgs_sink.clone();
        seg_scheduler.schedule_event(Box::new(move ||{
            seg_input_dist.start_dist(my_to_data, my_to_net);
        }), welcome_info.you_initialize_frame);

        self.init_net_rec_handling(set_net.net_rec, seg_data_storage.logic_msgs_sink);


        loop{
            thread::sleep(Duration::from_millis(10000));
        }
    }
}




pub fn client_main(connection_target_ip: String){
    println!("Starting as client.");
    let client = Client{
        player_name: String::from("Atomserdiah"),
        connection_ip: connection_target_ip,
    };
    client.start();
}





























