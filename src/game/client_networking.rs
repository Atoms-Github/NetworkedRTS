
use crate::network::networking_structs::*;
use crate::network::networking_message_types::*;
use crate::network::networking_utils::*;
use std::net::SocketAddr;


use tokio::codec::{FramedRead, FramedWrite};
use tokio::io::{lines, write_all};
use tokio::net::{TcpListener, TcpStream};
use tokio_io::*;


use std::io::{BufReader, Write};


pub struct HandshakeResponse{
    pub player_id: PlayerID
}


pub fn perform_handshake(target_ip : &String) -> HandshakeResponse{
    println!("Initializing connection to {}", target_ip);

    let mut connection = stall_thread_until_connection_success(target_ip);

    println!("Successfully made contact with {}! Sending initialization data.", target_ip);

    let connection_init_query = NetMessageType::ConnectionInitQuery(
        NetMsgConnectionInitQuery{
            my_player_name: "Atomsadiah!".to_string()
        }
    );

    let (mut read_half, mut write_half) = connection.split();

    let connection_init_bytes = bincode::serialize(&connection_init_query).unwrap();
    write_half.write(&connection_init_bytes[..]);




    HandshakeResponse{
        player_id: -1
    }
}















