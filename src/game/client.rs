use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::game::graphical_segment::GraphicalSegment;
use crate::game::logic_segment::LogicSegment;
use crate::network::game_message_types::NewPlayerInfo;
use crate::network::networking_message_types::*;
use crate::network::networking_segment::NetworkingSegment;
use crate::network::networking_structs::*;
use crate::network::game_message_types::*;
use crate::players::inputs::*;
use crate::game::channel_interchange::*;
use crate::game::timekeeping::*;
use std::panic;
use std::sync::{Arc, Mutex};


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
//    logic_segment.load_frames(welcome_info.frames_gathered_so_far); TODO: Implement.
//    logic_segment.add_new_player(&NewPlayerInfo{
//        player_id: welcome_info.assigned_player_id,
//        frame_added: welcome_info.known_frame_info.get_intended_current_frame() + 19
//    });
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

fn init_yeeting_my_inputs(inputs_stream: Receiver<InputChange>, outgoing_network: &Sender<NetMessageType>, to_logic: &Sender<LogicInwardsMessage>, welcome_info: &NetMsgConnectionInitResponse){
    let my_net_known_frame = welcome_info.known_frame_info.clone();
    let my_outgoing_network = outgoing_network.clone();
    let my_to_logic = to_logic.clone();
    let (for_network_sink, for_network_rec) = channel();
    let my_player_id = welcome_info.assigned_player_id;
    thread::spawn(move ||{
        gather_inputs_and_yeet_loop(for_network_rec, my_outgoing_network, my_player_id, my_net_known_frame);
    });
    let my_logic_known_frame = welcome_info.known_frame_info.clone();
    thread::spawn(move ||{
        loop{
            let inputs_change = inputs_stream.recv().unwrap();
            for_network_sink.send(inputs_change.clone()).unwrap(); // Apply to network.
            let message = LogicInwardsMessage::InputsUpdate(PlayerInputsSegmentResponse{
                player_id: my_player_id,
                start_frame_index: 2,
                input_states: vec![]
            });
            my_to_logic.send(message).unwrap();
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

    init_yeeting_my_inputs(player_inputs_rec,&outgoing_net_sink,&to_logic_sink, &welcome_info);


}





























