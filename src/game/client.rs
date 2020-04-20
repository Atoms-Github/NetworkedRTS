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
    fn init_graphics(&self, state_to_render: Arc<RwLock<GameState>>, my_player_id: PlayerID){
        let seg_graph = GraphicalSegment::new(state_to_render, my_player_id);
        seg_graph.start();
    }
    fn init_input_distribution(inputs_stream: Receiver<InputChange>, outgoing_network: Sender<NetMessageType>, to_logic: Sender<LogicInwardsMessage>,
                               welcome_info: &NetMsgConnectionInitResponse) -> InputDistributor{

        // TODO3: Things can be improved by not waiting for the entire frame to finish before sending the entire input frame to local logic. Could be as it comes.
    }

    fn start(self){
        let mut set_net = self.init_networking(&self.connection_ip);
        set_net.send_greeting(&self.player_name);
        let welcome_info = set_net.receive_welcome_message();

        let seg_data_storage = self.init_data_store(welcome_info.frames_gathered_so_far);
        let seg_tailer = self.init_tail_sim(welcome_info.known_frame_info.clone(), welcome_info.game_state, seg_data_storage.clone_lock_ref());
        let seg_header = self.init_head_sim(welcome_info.known_frame_info, seg_tailer.tail_lock, seg_data_storage.clone_lock_ref());
        let seg_graphics = self.init_graphics(seg_header.head_lock, welcome_info.assigned_player_id);


        self.init_net_rec_handling(set_net.net_rec, seg_data_storage.logic_msgs_sink);


        loop{
            thread::sleep(Duration::from_millis(10000)); // TODO1
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


    init_inwards_net_handling(net_rec, to_logic_sink.clone());


//    let mut graphical_segment = init_graphics(render_state_head, welcome_info.assigned_player_id);
//    let mut player_inputs_rec = graphical_segment.start();

    let input_distributor = init_input_distribution(player_inputs_rec, net_sink.clone(), to_logic_sink.clone(), &welcome_info);

    init_logic_output_responder(from_logic_rec, net_sink.clone(), input_distributor);
    // Now we wait for us to be initialized.


}





























