use std::net::SocketAddr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{SystemTime};

use crate::game::logic_segment::LogicSegment;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_hub_segment::{DistributableNetMessage, NetworkingHub, OwnedNetworkMessage};
use crate::network::networking_structs::*;

struct ServerMainState {
    all_frames: InputFramesStorage,
    big_fat_zero_time: KnownFrameInfo,
    outgoing_messages: Sender<DistributableNetMessage>,
    incoming_messages: Receiver<OwnedNetworkMessage>
}

pub fn server_main(hosting_ip: &String){
    println!("Starting as server. Going to host on {}", hosting_ip);

    let server = ServerMainState::init_server_hosting(hosting_ip);
    server.server_logic_loop();

    println!("Server finished.");
}

impl ServerMainState{
    pub fn init_server_hosting(hosting_ip: &String) -> ServerMainState{
        // Init connection.
        let addr = hosting_ip.to_string().parse::<SocketAddr>().unwrap();
        let mut networking_hub_segment = NetworkingHub::new();
        let (mut outgoing_sender, outgoing_receiver) = channel();
        let incoming_messages =
            networking_hub_segment.start_logic(outgoing_receiver, addr);

        // Init logic.
        let big_fat_zero_time = KnownFrameInfo{
            known_frame_index: 0,
            time: SystemTime::now()
        };
        let mut game_state = GameState::new();
        game_state.init_rts();
        let (mut logic_segment, mut state_handle) =
            LogicSegment::new(false, big_fat_zero_time.clone(), game_state);


        return ServerMainState{
            all_frames: InputFramesStorage::new(),
            big_fat_zero_time,

            outgoing_messages: outgoing_sender,
            incoming_messages
        }
    }
    pub fn server_logic_loop(mut self){

//        NetMessageType::ConnectionInitQuery(response) => {
//            let time = SystemTime::now();
//
//            let state_to_send = self.game_state_tail.clone(); // TODO this shouldn't need to be cloned to be serialized.
//            let frames_partial = self.all_frames.get_frames_partial(state_to_send.last_frame_simed + 1);
//            let response = NetMessageType::ConnectionInitResponse(NetMsgConnectionInitResponse{
//                assigned_player_id: *player_id,
//                frames_gathered_so_far: frames_partial,
//                known_frame_info: KnownFrameInfo { frame_index: state_to_send.last_frame_simed, time },
//                game_state: state_to_send,
//            });
//            let bytes = bincode::serialize(&response).unwrap();
//
////                            println!("Sending init message to client: {:?} {:?}", bytes, response);
//            println!("Init message bytes size: {}", bytes.len());
//            client_handle.write_channel.write(&bytes[..]).unwrap();
//        },
//        NetMessageType::InputsUpdate(updates) => {
//            input_updates.push((*updates).clone())
//        },
    }
}








