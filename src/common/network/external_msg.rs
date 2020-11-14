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

pub fn start_inwards_codec_thread_tcp(mut read_stream :TcpStream) -> Receiver<ExternalMsg>{
    let (sink, rec) = unbounded::<ExternalMsg>();
    thread::Builder::new().name("StreamDeserializerTCP".to_string()).spawn(move ||{
        loop{
//            let mut message_size_buffer = [0; 2];
//            let message_size_bytes = read_stream.read_exact(&mut message_size_buffer).unwrap();
//            let message_size = byteorder::LittleEndian::read_u16(&message_size_buffer);

            let mut message_buffer = vec![0; 65_535];
            read_stream.read(&mut message_buffer).unwrap();

            let result = bincode::deserialize::<ExternalMsg>(&message_buffer[..]);
            match result{
                Ok(msg) => {
                    if crate::DEBUG_MSGS_NET{
                        println!("<- {:?}", msg);
                    }
                    sink.send(msg).unwrap();
                }
                err => {
                    panic!("Err {:?}", err)
                }
            }
        }
    }).unwrap();
    rec
}

pub fn start_inwards_codec_thread_udp(mut read_stream :UdpSocket) -> Receiver<(ExternalMsg, SocketAddr)>{
    let (sender, receiver) = unbounded();
    thread::Builder::new().name("StreamDeserializerUDP".to_string()).spawn(move ||{
        let mut message_buffer = [0; 65_535];
        loop{
            let (message_size_bytes, address) = read_stream.recv_from(&mut message_buffer).unwrap();

            let result = bincode::deserialize::<ExternalMsg>(&message_buffer[..]);
            match result{
                Ok(msg) => {
                    if crate::DEBUG_MSGS_NET{
                        println!("<- {:?}", msg);
                    }
                    sender.send((msg, address)).unwrap();
                }
                err => {
                    panic!("Err {:?}", err)
                }
            }
        }
    }).unwrap();
    receiver
}


pub fn start_inwards_codec_thread_udp_filtered(mut read_stream :UdpSocket, filter_address: SocketAddr) -> Receiver<ExternalMsg>{ // This also ignores msgs from wrong address.
    let (sender, receiver) = unbounded();

    let unfiltered = start_inwards_codec_thread_udp(read_stream);
    thread::Builder::new().name("StreamDeserializerUDPFiltered".to_string()).spawn(move ||{
        loop{
            let (new_msg, addr) = unfiltered.recv().unwrap();
            assert_eq!(addr, filter_address, "Got message from wrong address.");
            sender.send(new_msg).unwrap();
        }
    }).unwrap();
    receiver
}



impl ExternalMsg{
//    pub fn encode_and_send_tcp(&self, write_stream :&mut TcpStream){ If this is needed, need to match udp. (Need to include size in main msg)
//        let connection_init_bytes = bincode::serialize(self).unwrap();
//        let message_size = connection_init_bytes.len() as u16;
//
//        let mut buffer = [0; 2];
//        byteorder::LittleEndian::write_u16(&mut buffer, message_size);
//        write_stream.write_all(&buffer).unwrap();
//        write_stream.write_all(&connection_init_bytes).unwrap();
//        write_stream.flush().unwrap();
//
//        if crate::DEBUG_MSGS_NET{
//            println!("->: {:?}", self);
//        }
//    }

    pub fn encode_and_send_udp(&self, write_stream :&mut UdpSocket, address: SocketAddr){
        let msg_buffer = bincode::serialize(self).unwrap();

        write_stream.send_to(&msg_buffer, address).unwrap();

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






