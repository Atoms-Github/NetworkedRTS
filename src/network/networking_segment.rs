use ggez::*;
use ggez::{ContextBuilder, event};
use futures::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::network::networking_structs::*;
use crate::network::networking_message_types::*;
use crate::players::inputs::*;
use ggez::event::{EventHandler, KeyMods};
use ggez::input::keyboard::KeyCode;
use crate::game::client_networking::connect_and_send_handshake;

use crate::systems::render::*;
use futures::future::lazy;

use crate::ecs::world::*;
use crate::ecs::system_macro::*;
use crate::network::*;

use crate::game::client_networking::*;

use futures::future::Future;

use std::time::{SystemTime};
use std::io::Write;
use bytes::Bytes;
use futures::sink::Sink;
use std::net::{TcpStream, SocketAddr};
use std::thread::Thread;
use crate::game::logic_segment;
use std::sync::mpsc::Receiver;

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
    pub fn init_connection(&mut self, player_name: String) -> Receiver<NetMessageType>{
        let connection_result = TcpStream::connect(&self.connection_address);
        let mut stream = connection_result.expect("Failed to connect.");

//        let mut read_stream = stream.try_clone().unwrap();
//        let mut write_stream = stream;

        let connection_init_query = NetMessageType::ConnectionInitQuery(
            NetMsgConnectionInitQuery{
                my_player_name: player_name
            }
        );
        connection_init_query.encode_and_send(&mut stream);

        return start_inwards_codec_thread(stream);
    }
}












