use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::net::SocketAddr;
use std::str::FromStr;

use crate::network::networking_message_types::*;

pub struct NetworkingSegment {
//    socket: Option<TcpStream>,
    pub net_sink: Sender<NetMessageType>,
    pub net_rec: Receiver<NetMessageType>,
}

impl NetworkingSegment {
    pub fn send_greeting(&mut self, player_name: &String){
        let connection_init_query = NetMessageType::ConnectionInitQuery(
            NetMsgConnectionInitQuery{
                my_player_name: player_name.clone()
            }
        );
        self.net_sink.send(connection_init_query).unwrap();
    }
    pub fn init_connection(conn_address_str: &String) -> NetworkingSegment{
        let conn_address = SocketAddr::from_str(conn_address_str).expect("Ill formed ip");
        let connection_result = TcpStream::connect(conn_address);
        let mut stream = connection_result.expect("Failed to connect.");

        let (out_sink, out_rec) = channel();
        let mut stream_outgoing = stream.try_clone().unwrap();
        thread::spawn(move ||{
            loop{
                let message_to_send: NetMessageType = out_rec.recv().unwrap();
                message_to_send.encode_and_send(&mut stream_outgoing);
            }
        });
        let in_rec = start_inwards_codec_thread(stream);
        return NetworkingSegment{
            net_sink: out_sink,
            net_rec: in_rec
        };
    }
}












