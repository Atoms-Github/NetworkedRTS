
use serde::{Serialize, Deserialize};
use std::{fmt, thread};
use crate::network::networking_structs::*;
use crate::players::inputs::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::mpsc::{Receiver, Sender, channel};
use bytes::Reader;
use std::io::{Read, Write};
use byteorder::ByteOrder;
use std::net::TcpStream;
use crate::network::game_message_types;
use crate::game::timekeeping::KnownFrameInfo;


pub fn start_inwards_codec_thread(mut read_stream :TcpStream) -> Receiver<NetMessageType>{ // TODO: Investigate a way to destroy thread when receiver is dropped.
    let (sender, receive) = channel::<NetMessageType>();
    thread::spawn(move ||{
        loop{
            let mut message_size_buffer = [0; 2];
            let message_size_bytes = read_stream.read_exact(&mut message_size_buffer).unwrap();
            let message_size = byteorder::LittleEndian::read_u16(&message_size_buffer);

            let mut message_buffer = vec![0; message_size as usize];
            read_stream.read_exact(&mut message_buffer);

            let result = bincode::deserialize::<NetMessageType>(&message_buffer[..]); // TODO should crash on failure.
            match result{
                Ok(msg) => {
                    sender.send(msg);
                }
                err => {
                    panic!("Err {:?}", err)
                }
            }
        }
    });
    return receive;
}

impl NetMessageType{
    pub fn encode_and_send(&self, mut write_stream :&TcpStream){
        let connection_init_bytes = bincode::serialize(self).unwrap();
        let message_size = connection_init_bytes.len() as u16;

        let mut buffer = [0; 2];
        byteorder::LittleEndian::write_u16(&mut buffer, message_size);
        write_stream.write(&buffer).unwrap();
        write_stream.write(&connection_init_bytes).unwrap();
        write_stream.flush();
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)] // Serializing and deserializing enums with data does store which enum it is - we don't need to store the data and enum separately.
pub enum NetMessageType {
    ConnectionInitQuery(NetMsgConnectionInitQuery),
    GameUpdate(game_message_types::GameMessageType),
    ConnectionInitResponse(NetMsgConnectionInitResponse),
    LocalCommand(LocalCommandInfo)
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalCommandInfo{
    pub command: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgConnectionInitQuery {
    pub my_player_name: String
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgConnectionInitResponse {
    pub assigned_player_id: PlayerID,
    pub game_state: GameState,
    pub frames_gathered_so_far: FramesStoragePartial,
    pub known_frame_info: KnownFrameInfo
}

























