use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::game::graphical_segment::GraphicalSegment;
use crate::network::networking_message_types::*;
use crate::network::networking_segment::NetworkingSegment;
use crate::network::networking_structs::*;
use crate::players::inputs::*;
use std::panic;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use crate::game::logic::logic_segment::*;
use crate::game::synced_data_stream::*;
use crate::game::timekeeping::*;
use crate::game::logic::logic_data_storage::*;
use crate::game::logic::data_storage_manager::*;


struct Client{
    player_name: String
}
impl Client{
    fn init_networking(&self, connection_target_ip: &String, player_name: &String) ->
           (Sender<NetMessageType>, Receiver<NetMessageType>, NetMsgConnectionInitResponse){
        let ip = SocketAddr::from_str(connection_target_ip).expect("Ill formed ip");
        let mut networking_seg = NetworkingSegment::new(ip);
        let (mut outgoing_messages, mut incoming_messages) = networking_seg.init_connection(player_name);

        let welcome_message = incoming_messages.recv().unwrap();
        match welcome_message{
            NetMessageType::ConnectionInitResponse(info) =>{
                return (outgoing_messages, incoming_messages, info);
            }
            _ => {
                panic!("First message read wasn't welcome.");
            }
        }
    }
}



fn init_logic_tail(welcome_info: NetMsgConnectionInitResponse) -> (Sender<LogicInwardsMessage>, Receiver<LogicOutwardsMessage>, Arc<Mutex<GameState>>){
    let (from_logic_sink, from_logic_rec) = channel();
    let (to_logic_sink, to_logic_rec) = channel();


    let tail_lock = Arc::new(RwLock::new(welcome_info.game_state));
    let storage_manager = DataStorageManager::new(welcome_info.frames_gathered_so_far);


    let mut logic_segment = LogicSegmentTailer::new(welcome_info.known_frame_info.clone()
                                                    , tail_lock, from_logic_sink, storage_manager.clone_lock_ref());
    logic_segment.start_logic_thread();



    return (to_logic_sink, from_logic_rec, state_head);
}

fn init_graphics(state_to_render: Arc<Mutex<GameState>>, my_player_id: PlayerID) -> GraphicalSegment{
    return GraphicalSegment::new(state_to_render, my_player_id);
}

struct InputDistributor{
    inputs_stream_state: Receiver<InputState>,
    outgoing_network: Sender<NetMessageType>,
    to_logic: Sender<LogicInwardsMessage>,
    known_frame: KnownFrameInfo,
    player_id: PlayerID
}
impl InputDistributor{
    fn start(self){
        thread::spawn(move ||{
            loop{
                let state = self.inputs_stream_state.recv().unwrap();
                let now_frame_index = self.known_frame.get_intended_current_frame(); // Super important this doesn't change between local and sent so we get here.

                let logic_message = LogicInwardsMessage::SyncerInputsUpdate(SyncerData{
                    data: vec![],
                    start_frame: now_frame_index,
                    owning_player: self.player_id as i32
                });
                self.to_logic.send(logic_message.clone()).unwrap();
                self.outgoing_network.send(NetMessageType::GameUpdate(logic_message)).unwrap();
            }
        });
    }
}
fn init_input_distribution(inputs_stream: Receiver<InputChange>, outgoing_network: Sender<NetMessageType>, to_logic: Sender<LogicInwardsMessage>,
                           welcome_info: &NetMsgConnectionInitResponse) -> InputDistributor{
    let changes_rec = init_input_collector_thread(inputs_stream, welcome_info.known_frame_info.clone());
    InputDistributor{
        inputs_stream_state: changes_rec,
        outgoing_network,
        to_logic,
        known_frame: welcome_info.known_frame_info.clone(),
        player_id: welcome_info.assigned_player_id
    }

    // TODO3: Things can be improved by not waiting for the entire frame to finish before sending the entire input frame to local logic. Could be as it comes.
}

fn init_logic_output_responder(logic_output: Receiver<LogicOutwardsMessage>, network_sink: Sender<NetMessageType>, input_distributor: InputDistributor){
    thread::spawn(move || {
        let mut my_distributor = Some(input_distributor);
        loop{
            let logic_msg = logic_output.recv().unwrap();
            match logic_msg{

                LogicOutwardsMessage::DataNeeded(syncer_request) => {
//                    network_sink.send(NetMessageType::())
                    // TODO1: Implement
                }
                LogicOutwardsMessage::IAmInitialized() => {
                    my_distributor.take().unwrap().start();
                }
            }
        }
    });
}

fn init_inwards_net_handling(incoming_messages: Receiver<NetMessageType>, to_logic: Sender<LogicInwardsMessage>){
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

pub fn client_main(connection_target_ip: &String){
    println!("Starting as client.");

    let (mut net_sink, mut net_rec, welcome_info) =
        init_networking(connection_target_ip, &(String::from("Atomsadiah")));

    let (mut to_logic_sink, mut from_logic_rec, mut render_state_head) =
        init_logic_tail(welcome_info.clone());


    init_inwards_net_handling(net_rec, to_logic_sink.clone());


    let mut graphical_segment = init_graphics(render_state_head, welcome_info.assigned_player_id);
    let mut player_inputs_rec = graphical_segment.start();

    let input_distributor = init_input_distribution(player_inputs_rec, net_sink.clone(), to_logic_sink.clone(), &welcome_info);

    init_logic_output_responder(from_logic_rec, net_sink.clone(), input_distributor);
    // Now we wait for us to be initialized.

    loop{
        thread::sleep(Duration::from_millis(10000)); // TODO1
    }
}





























