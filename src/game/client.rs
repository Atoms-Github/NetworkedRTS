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
use std::net::TcpStream;
use std::thread::Thread;
use crate::game::game_logic_layer;






pub fn client_main(connection_target_ip: &String){
    let local_connection_target_ip = connection_target_ip.clone();
    println!("Starting as client.");

    let server_connection_init = connect_and_send_handshake(&local_connection_target_ip);


}


