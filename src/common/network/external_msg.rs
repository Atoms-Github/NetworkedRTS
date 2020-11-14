use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket, SocketAddr};
use std::thread;
use std::time::SystemTime;

use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

use crate::common::gameplay::game::game_state::*;
use crate::common::logic::logic_sim_tailer_seg::*;
use crate::common::time::timekeeping::*;
use crate::common::types::*;
use crate::common::sim_data::sim_data_storage::*;
use std::intrinsics::add_with_overflow;
use crate::common::data::hash_seg::FramedHash;
use crossbeam_channel::*;

pub trait GameSocket{
    fn start_listening(self, msgs_sink: Sender<(ExternalMsg, SocketAddr)>);
}
pub trait GameSocketTcp{
    fn send_msg(&mut self, message: &ExternalMsg);
}
pub trait GameSocketUdp{
    fn send_msg(&mut self, message: &ExternalMsg, addr: &SocketAddr);
}
impl GameSocketTcp for TcpStream{
    fn send_msg(&mut self, message: &ExternalMsg) {
        let connection_init_bytes = bincode::serialize(message).unwrap();
        self.write_all(&connection_init_bytes).unwrap();
        self.flush().unwrap();

        if crate::DEBUG_MSGS_NET{
            println!("->: {:?}", self);
        }
    }
}
impl GameSocket for TcpStream{
    fn start_listening(mut self, msgs_sink: Sender<(ExternalMsg, SocketAddr)>) {
        thread::Builder::new().name("StreamDeserializerTCP".to_string()).spawn(move ||{
            let peer_address = self.peer_addr().unwrap();
            loop{
                let mut message_buffer = vec![0; 65_535];
                let value = self.read(&mut message_buffer).unwrap();
                let result = bincode::deserialize::<ExternalMsg>(&message_buffer[..]);
                match result{
                    Ok(msg) => {
                        if crate::DEBUG_MSGS_NET{
                            println!("<- {:?}", msg);
                        }
                        msgs_sink.send((msg, peer_address.clone())).unwrap();
                    }
                    err => {
                        panic!("Err {:?}", err)
                    }
                }
            }
        }).unwrap();
    }
}
impl GameSocket for UdpSocket{
    fn start_listening(self, msgs_sink: Sender<(ExternalMsg, SocketAddr)>) {
        thread::Builder::new().name("StreamDeserializerUDP".to_string()).spawn(move ||{
            let mut message_buffer = [0; 65_535];
            loop{
                let (message_size_bytes, address) = self.recv_from(&mut message_buffer).unwrap();

                let result = bincode::deserialize::<ExternalMsg>(&message_buffer[..]);
                match result{
                    Ok(msg) => {
                        if crate::DEBUG_MSGS_NET{
                            println!("<- {:?}", msg);
                        }
                        msgs_sink.send((msg, address)).unwrap();
                    }
                    err => {
                        panic!("Err {:?}", err)
                    }
                }
            }
        }).unwrap();
    }
}
impl GameSocketUdp for UdpSocket{
    fn send_msg(&mut self, message: &ExternalMsg, address: &SocketAddr) {
        let msg_buffer = bincode::serialize(message).unwrap();

        self.send_to(&msg_buffer, address).unwrap();

        if crate::DEBUG_MSGS_NET{
            println!("->({}): {:?}", msg_buffer.len(), self);
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)] // Serializing and deserializing enums with data does store which enum it is - we don't need to store the data and enum separately.
pub enum ExternalMsg {
    ConnectionInitQuery(NetMsgGreetingQuery),
    ConnectionInitResponse(NetMsgGreetingResponse),
    NewHash(FramedHash),
    GameUpdate(OwnedSimData),
    LocalCommand(LocalCommandInfo),
    PingTestQuery(SystemTime),
    PingTestResponse(NetMsgPingTestResponse),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgPingTestResponse{
    pub client_time: SystemTime,
    pub server_time: SystemTime,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalCommandInfo{
    pub command: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgGreetingQuery {
    pub my_player_name: String,
    pub preferred_id: i32
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgGreetingResponse {
    pub assigned_player_id: PlayerID,
    pub game_state: GameState,
    pub players_in_state: Vec<PlayerID>,
    pub known_frame: KnownFrameInfo,
}






