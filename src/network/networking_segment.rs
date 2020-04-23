use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::str::FromStr;

use crate::network::networking_message_types::*;

pub struct NetworkingSegmentIn {
    conn_address_str: String
}
pub struct NetworkingSegmentEx {
    pub net_sink: Sender<NetMessageType>,
    pub net_rec: Receiver<NetMessageType>,
}
impl NetworkingSegmentEx {
    pub fn send_greeting(&mut self, player_name: &String){
        let connection_init_query = NetMessageType::ConnectionInitQuery(
            NetMsgConnectionInitQuery{
                my_player_name: player_name.clone()
            }
        );
        self.net_sink.send(connection_init_query).unwrap();
    }
    pub fn receive_welcome_message(&mut self) -> NetMsgConnectionInitResponse{
        loop{
            let welcome_message = self.net_rec.recv().unwrap();
            match welcome_message{
                NetMessageType::ConnectionInitResponse(info) =>{
                    return info;
                }
                _ => {
                }

            }
            println!("Ignoring first messages until welcome info: {:?}", welcome_message);
        }
    }
}

impl NetworkingSegmentIn {
    pub fn new(conn_address_str :String) -> NetworkingSegmentIn{
        NetworkingSegmentIn{
            conn_address_str
        }
    }
    pub fn start_net(self) -> NetworkingSegmentEx {
        let conn_address = SocketAddr::from_str(&self.conn_address_str).expect("Ill formed ip");
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
        return NetworkingSegmentEx {
            net_sink: out_sink,
            net_rec: in_rec
        };
    }
}












