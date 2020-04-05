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
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::game::logic::logic_segment::*;
use crate::game::synced_data_stream::*;


fn init_networking(connection_target_ip: &String, player_name: &String) -> (Sender<NetMessageType>, Receiver<NetMessageType>, NetMsgConnectionInitResponse){
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

fn init_logic(welcome_info: NetMsgConnectionInitResponse) -> (Sender<LogicInwardsMessage>, Receiver<LogicOutwardsMessage>, Arc<Mutex<GameState>>){
    let (mut from_logic_sink, mut from_logic_rec) = channel();
    let (mut logic_segment, mut state_head) = LogicSegment::new(
        true, welcome_info.known_frame_info.clone(), welcome_info.game_state, from_logic_sink);

    logic_segment.load_frames(welcome_info.frames_gathered_so_far); // TODO3: A bit clumsy.
    let (to_logic_sink, to_logic_rec) = channel();
    thread::spawn(move ||{
        logic_segment.run_logic_loop(to_logic_rec);
    });
    return (to_logic_sink, from_logic_rec, state_head);
}

fn init_graphics(state_to_render: Arc<Mutex<GameState>>, my_player_id: PlayerID) -> Receiver<InputChange>{
    let mut graphics_segment = GraphicalSegment::new(state_to_render, my_player_id);
    return graphics_segment.start();
}

fn init_input_distribution(inputs_stream: Receiver<InputChange>, outgoing_network: Sender<NetMessageType>, to_logic: Sender<LogicInwardsMessage>, welcome_info: &NetMsgConnectionInitResponse){
    let changes = init_input_collector_thread(inputs_stream, welcome_info.known_frame_info.clone());
    // TODO3: Things can be improved by not waiting for the entire frame to finish before sending the entire input frame to local logic. Could be as it comes.

    let my_known_info = welcome_info.known_frame_info.clone();
    let my_player_id = welcome_info.assigned_player_id;
    thread::spawn(move ||{
        loop{
            let state = changes.recv().unwrap();
            let now_frame_index = my_known_info.get_intended_current_frame(); // Super important this doesn't change between local and sent so we get here.

            let logic_message = LogicInwardsMessage::SyncerInputsUpdate(SyncerData{
                data: vec![],
                start_frame: now_frame_index,
                owning_player: my_player_id as i32
            });
            to_logic.send(logic_message.clone()).unwrap();
            outgoing_network.send(NetMessageType::GameUpdate(logic_message)).unwrap();
        }
    });
}

pub fn client_main(connection_target_ip: &String){
    println!("Starting as client.");

    let (mut outgoing_net_sink, mut incoming_net_rec, welcome_info) =
        init_networking(connection_target_ip, &(String::from("Atomsadiah")));

    let (mut to_logic_sink, mut from_logic_rec, mut render_state_head) =
        init_logic(welcome_info.clone());

    let mut player_inputs_rec = init_graphics(render_state_head, welcome_info.assigned_player_id);

    init_input_distribution(player_inputs_rec, outgoing_net_sink.clone(), to_logic_sink.clone(), &welcome_info);

    loop{
        thread::sleep(Duration::from_millis(10)); // TODO1
    }
}





























