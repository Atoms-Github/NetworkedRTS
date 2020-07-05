use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::SystemTime;

use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

use crate::common::gameplay::game::game_state::*;
use crate::common::logic::logic_sim_tailer_seg::*;
use crate::common::time::timekeeping::*;
use crate::common::types::*;

pub fn start_inwards_codec_thread(mut read_stream :TcpStream) -> Receiver<ExternalMsg>{
    let (sender, receive) = channel::<ExternalMsg>();
    thread::Builder::new().name("StreamDeserializer".to_string()).spawn(move ||{
        loop{
            let mut message_size_buffer = [0; 2];
            let message_size_bytes = read_stream.read_exact(&mut message_size_buffer).unwrap();
            let message_size = byteorder::LittleEndian::read_u16(&message_size_buffer);

            let mut message_buffer = vec![0; message_size as usize];
            read_stream.read_exact(&mut message_buffer).unwrap();

            let result = bincode::deserialize::<ExternalMsg>(&message_buffer[..]);
            match result{
                Ok(msg) => {
                    if crate::DEBUG_MSGS_NET{
                        println!("<- {:?}", msg);
                    }
                    sender.send(msg).unwrap();
                }
                err => {
                    panic!("Err {:?}", err)
                }
            }
        }
    }).unwrap();
    receive
}

impl ExternalMsg{
    pub fn encode_and_send(&self, write_stream :&mut TcpStream){
        let connection_init_bytes = bincode::serialize(self).unwrap();
        let message_size = connection_init_bytes.len() as u16;

        let mut buffer = [0; 2];
        byteorder::LittleEndian::write_u16(&mut buffer, message_size);
        write_stream.write_all(&buffer).unwrap();
        write_stream.write_all(&connection_init_bytes).unwrap();
        write_stream.flush().unwrap();

        if crate::DEBUG_MSGS_NET{
            println!("->: {:?}", self);
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)] // Serializing and deserializing enums with data does store which enum it is - we don't need to store the data and enum separately.
pub enum ExternalMsg {
    ConnectionInitQuery(NetMsgGreetingQuery),
    ConnectionInitResponse(NetMsgGreetingResponse),
    GameUpdate(LogicInwardsMessage),
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
    pub my_player_name: String
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgGreetingResponse {
    pub assigned_player_id: PlayerID,
    pub game_state: GameState,
    pub known_frame: KnownFrameInfo,
}






