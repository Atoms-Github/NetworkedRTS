use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::network::networking_message_types::*;

pub struct NetworkingSegment {
//    socket: Option<TcpStream>,
    connection_address: SocketAddr
}

impl NetworkingSegment {
    pub fn new(conn_address: SocketAddr) -> NetworkingSegment {
        NetworkingSegment {
//            socket: None,
            connection_address: conn_address
        }
    }
    pub fn init_connection(&mut self, player_name: &String) -> (Sender<NetMessageType>, Receiver<NetMessageType>){
        let connection_result = TcpStream::connect(&self.connection_address);
        let mut stream = connection_result.expect("Failed to connect.");

        let (out_send, out_rec) = channel();
        let mut stream_outgoing = stream.try_clone().unwrap();
        thread::spawn(move ||{
            loop{
                let message_to_send: NetMessageType = out_rec.recv().unwrap();
                message_to_send.encode_and_send(&mut stream_outgoing);
            }
        });
        let connection_init_query = NetMessageType::ConnectionInitQuery(
            NetMsgConnectionInitQuery{
                my_player_name: player_name.clone()
            }
        );
        out_send.send(connection_init_query).unwrap();

        let in_rec = start_inwards_codec_thread(stream);
        return (out_send, in_rec);
    }
}












