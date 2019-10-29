
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


use std::iter;
use tokio::prelude::*; // 0.1.15



pub struct HandshakeResponse{
    pub player_id: PlayerID,
    pub socket_read: FramedRead<ReadHalf<TcpStream>, Bytes>,
    pub socket_write: WriteHalf<TcpStream>,
}


fn example() -> impl Stream<Item = i32, Error = ()> {
    stream::iter_ok(iter::repeat(42))
}



pub fn perform_handshake(target_ip : &String) -> HandshakeResponse{

    println!("Initializing connection to {}", target_ip);

    let mut connection = stall_thread_until_connection_success(target_ip);
    let mut connection_dcwct = stall_thread_until_connection_success(target_ip);

    println!("Successfully made contact with {}! Sending initialization data.", target_ip);

    let connection_init_query = NetMessageType::ConnectionInitQuery(
        NetMsgConnectionInitQuery{
            my_player_name: "Atomsadiah!".to_string()
        }
    );

    let (mut read_half, mut write_half) = connection.split();
    let (mut read_half_dcwct, mut write_half_dcwct) = connection_dcwct.split();

    let connection_init_bytes = bincode::serialize(&connection_init_query).unwrap();
    write_half.write(&connection_init_bytes[..]);


    let stream = FramedRead::new(read_half, dans_codec::Bytes);
    let stream_dcwct = FramedRead::new(read_half_dcwct, dans_codec::Bytes);


//    stream.readv(); TODO: Investigate these two options.
//    stream.take();


    let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
    let r = runtime.block_on(stream.into_future());
    if let Ok((v, _)) = r {
        println!("{:?}", v);
    }



//    let mut stream_iterator = stream.wait();
//    let meme = Iterator::next(&mut stream_iterator).unwrap();

//    let first_item_read : Vec<u8> = meme.unwrap(); // TODO: Test :) Not sure how well this will work.

    let first_item_read : Vec<u8> = vec![];
    let received = bincode::deserialize::<NetMessageType>(&first_item_read[..]).unwrap();


    let player_id;
    match received{
        NetMessageType::ConnectionInitResponse(response) => {
            println!("Successfully read handshake response!");
            player_id = response.assigned_player_id;
        },
        _ => {
            panic!("First item read from server after handshake request wasn't a handshake response!");
            //TODO let mut player_id = 0;
        },
    }

    HandshakeResponse{
        player_id,
        socket_read: stream_dcwct,
        socket_write: write_half,
    }

}















