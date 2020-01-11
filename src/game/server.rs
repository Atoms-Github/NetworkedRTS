use std::net::SocketAddr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{SystemTime};

use crate::game::logic_segment::LogicSegment;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_hub_segment::{DistributableNetMessage, NetworkingHub, OwnedNetworkMessage};
use crate::network::networking_structs::*;
use crate::network::networking_message_types::{NetMessageType, NetMsgConnectionInitResponse};
use std::sync::{Mutex, Arc};
use std::thread;
use crate::network::game_message_types::GameMessageType;
use std::panic;

struct ServerMainState {
    all_frames: InputFramesStorage,
    big_fat_zero_time: KnownFrameInfo,
    outgoing_messages: Sender<DistributableNetMessage>,
    incoming_messages: Receiver<OwnedNetworkMessage>,
    game_state_tail: Arc<Mutex<GameState>>,
    game_updates_sink: Sender<GameMessageType>
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
            networking_hub_segment.start_listening(outgoing_receiver, addr);

        // Init logic.
        let big_fat_zero_time = KnownFrameInfo{
            known_frame_index: 0,
            time: SystemTime::now()
        };
        let mut game_state = GameState::new();
        game_state.init_rts();
        let (mut logic_segment, mut state_handle) =
            LogicSegment::new(false, big_fat_zero_time.clone(), game_state);

        let (mut game_updates_sink, mut game_updates_rec) = channel();
        thread::spawn(||{
            logic_segment.run_logic_loop(game_updates_rec);
        });


        return ServerMainState{
            all_frames: InputFramesStorage::new(),
            big_fat_zero_time,

            outgoing_messages: outgoing_sender,
            incoming_messages,
            game_state_tail: state_handle,
            game_updates_sink
        }
    }
    pub fn server_logic_loop(mut self){

        loop{
            let incoming_owned_message = self.incoming_messages.recv().unwrap();

            let incoming_message = incoming_owned_message.message;
            let player_id = incoming_owned_message.owner;

            match incoming_message{
                NetMessageType::ConnectionInitQuery(response) => {
                    let state_to_send = self.game_state_tail.lock().unwrap().clone(); // TODO this shouldn't need to be cloned to be serialized.
                    let inputs_since_connection = self.all_frames.get_frames_partial(state_to_send.last_frame_simed + 1);
                    let response = NetMessageType::ConnectionInitResponse(NetMsgConnectionInitResponse{
                        assigned_player_id: player_id,
                        frames_gathered_so_far: inputs_since_connection,
                        known_frame_info: KnownFrameInfo { known_frame_index: state_to_send.last_frame_simed, time: SystemTime::now() },
                        game_state: state_to_send,
                    });
                    println!("Received initialization request for player with ID: {}", player_id);
                    self.outgoing_messages.send(DistributableNetMessage::ToSingle(player_id, response)).unwrap();
                },
                NetMessageType::GameUpdate(update_info) => {
                    self.game_updates_sink.send(update_info).unwrap();
                },
                _ => {
                    panic!("Unexpected message");
                }
            }
        }


    }
}








