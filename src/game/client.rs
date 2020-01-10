use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::thread;

use crate::game::graphical_segment::GraphicalSegment;
use crate::game::logic_segment::LogicSegment;
use crate::network::game_message_types::NewPlayerInfo;
use crate::network::networking_message_types::*;
use crate::network::networking_segment::NetworkingSegment;
use std::panic;

pub fn client_main(connection_target_ip: &String){
    let local_connection_target_ip = connection_target_ip.clone();
    println!("Starting as client.");

    let ip = SocketAddr::from_str(connection_target_ip).expect("Ill formed ip");
    let mut networking_seg = NetworkingSegment::new(ip);
    let (mut outoing_messages, mut incoming_messages) = networking_seg.init_connection("Atomsadiah".to_string());

    let welcome_message = incoming_messages.recv().unwrap();
    let welcome_info;
    match welcome_message{
        NetMessageType::ConnectionInitResponse(info) =>{
            welcome_info = info;
        }
        _ => {
            panic!("First message read wasn't welcome.");
        }
    }



    let (mut logic_segment, mut state_head) = LogicSegment::new(
        true, welcome_info.known_frame_info.clone(), welcome_info.game_state);
    logic_segment.load_frames(welcome_info.frames_gathered_so_far);
    logic_segment.add_new_player(&NewPlayerInfo{
        player_id: welcome_info.assigned_player_id,
        frame_added: welcome_info.known_frame_info.get_intended_current_frame() + 19
    });
    let (game_msg_send, game_msg_rec) = channel();
    thread::spawn(move ||{
        logic_segment.run_logic_loop(game_msg_rec);
    });
    let mut graphics_segment = GraphicalSegment::new(state_head, welcome_info.assigned_player_id);
    let inputs_rec = graphics_segment.start();


}





























