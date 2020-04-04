

use crate::players::inputs::*;
use crate::network::networking_message_types::*;
use crate::game::timekeeping::*;
use crate::network::networking_structs::*;
use crate::network::game_message_types::*;
use crate::game::server::*;
use crate::network::networking_hub_segment::*;



// TODO2: Move all interchanges to here.


use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

pub fn gather_inputs_and_yeet_loop(inputs_stream: Receiver<InputChange>, outgoing_network: Sender<NetMessageType>, player_id: PlayerID, known_frame: KnownFrameInfo){
    let mut frame_generator = known_frame.start_frame_stream();

    loop{
        let frame_index = frame_generator.recv().unwrap(); // Wait for new frame.
        let mut inputs_state = InputState::new();

        let mut change = inputs_stream.try_recv();
        while change.is_ok(){ // Keep fishing.
            change.unwrap().apply_to_state(&mut inputs_state);

            change = inputs_stream.try_recv();
        }


        let inputs_update_message = NetMessageType::GameUpdate(LogicInwardsMessage::InputsUpdate(LogicInputsResponse {
            player_id,
            start_frame_index: frame_index,

            input_states: vec![ PlayerInputSegmentType::WholeState(inputs_state) ] // For now, just send one input. Can be changed to 2 or 20 if lots of input packages are failing.
        }));
        outgoing_network.send(inputs_update_message).unwrap();
    }
}

pub fn gather_incoming_server_messages(inc_clients: Receiver<OwnedNetworkMessage>, bonus_msgs: Receiver<BonusMsgsResponse>)
-> Receiver<ServerActableMessage>{
    let (actable_sink,actable_rec) = channel();

    let actable_from_clients = actable_sink.clone();
    thread::spawn(move ||{
        loop{
            let client_message = inc_clients.recv().unwrap();
            actable_from_clients.send(ServerActableMessage::IncomingClientMsg(client_message)).unwrap();
        }
    });
    let actable_from_clients = actable_sink.clone();
    thread::spawn(move ||{
        loop{
            let bonus_msg = bonus_msgs.recv().unwrap();
            actable_from_clients.send(ServerActableMessage::NewlyGeneratedBonusMsgs(bonus_msg)).unwrap();
        }
    });
    return actable_rec;
}