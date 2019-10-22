
use crate::network::networking_structs::*;
use crate::network::networking_message_types::*;
use crate::network::networking_utils::*;


use tokio::codec::{FramedRead, FramedWrite};
use tokio::io::{lines, write_all, ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio_io::*;


use std::io::{BufReader, Write};

use crate::network::*;
use futures::stream::Stream;
use futures::future::Future;
use crate::network::dans_codec::Bytes;


pub struct HandshakeResponse{
    pub player_id: PlayerID,
    pub socket_read: FramedRead<ReadHalf<TcpStream>, Bytes>,
    pub socket_write: WriteHalf<TcpStream>,
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


    let stream = FramedRead::new(read_half, dans_codec::Bytes);

    let mut stream_iterator = stream.wait();
    let first_item_read : Vec<u8> = Iterator::next(&mut stream_iterator).unwrap().unwrap(); // Not sure how well this will work. :)

    

    let received = bincode::deserialize::<NetMessageType>(&first_item_read[..]).unwrap();



    let mut player_id = -1;
    match received{
        NetMessageType::ConnectionInitResponse(response) => {
            println!("Successfully read handshake response!");
            player_id = response.assigned_player_id;
        },
        _ => {
            println!("First item read from server after handshake request wasn't a handshake response!");
        },
    }

    HandshakeResponse{
        player_id,
        socket_read: stream,
        socket_write: write_half,
    }
}















