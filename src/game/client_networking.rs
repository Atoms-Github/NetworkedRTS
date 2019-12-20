
use crate::network::networking_structs::*;
use crate::network::networking_message_types::*;
use crate::network::networking_utils::*;



use std::io::{BufReader, Write};

use crate::network::*;
use futures::stream::Stream;
use futures::future::Future;




use std::{iter, thread};
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use byteorder::{ByteOrder, LittleEndian};


pub struct ConnectToServerInit {
//    pub player_id: PlayerID,
//    pub welcome_messages_channel: Receiver<NetMessageType>,
//    pub normal_messages_channel: Receiver<NetMessageType>,
//    pub normal_messages_channel: FramedRead<ReadHalf<TcpStream>, Bytes>,
    pub write_stream: TcpStream,
    pub msg_box: MessageBox
}


//struct MyStream {
//    current: u32,
//    max: u32,
//}
//
//impl MyStream {
//    pub fn new(max: u32) -> MyStream {
//        MyStream {
//            current: 0,
//            max: max,
//        }
//    }
//}
//
//impl Stream for MyStream {
//    type Item = u32;
//    type Error = Box<Error>;
//
//    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
//        match self.current {
//            ref mut x if *x < self.max => {
//                *x = *x + 1;
//                Ok(Async::Ready(Some(*x)))
//            }
//            _ => Ok(Async::Ready(None)),
//        }
//    }
//}





pub fn connect_and_send_handshake(target_ip : &String) -> ConnectToServerInit {//Box<dyn Future<Item = HandshakeResponse, Error = ()> + Send>{
    println!("Initializing connection to {}", target_ip);
    let addr = target_ip.to_string().parse::<SocketAddr>().unwrap();

    let connection_result = TcpStream::connect(&addr);
    let stream = connection_result.expect("Failed to connect.");

    let mut read_stream = stream.try_clone().unwrap();
    let mut write_stream = stream;

    let connection_init_query = NetMessageType::ConnectionInitQuery(
        NetMsgConnectionInitQuery{
            my_player_name: "Atomsadiah!".to_string(),
            test_field: "Wubba".to_string(),
            test_two: 99
        }
    );
    let connection_init_bytes = bincode::serialize(&connection_init_query).unwrap();
    let message_size = connection_init_bytes.len() as u16;

    let mut buffer = [0; 10];
    byteorder::LittleEndian::write_u16(&mut buffer, message_size);
    write_stream.write(&buffer).unwrap();
    write_stream.write(&connection_init_bytes).unwrap();
    write_stream.flush();


    let mut msg_box = MessageBox::new();
    msg_box.spawn_thread_message_box_fill(read_stream);

    ConnectToServerInit {
        write_stream,
        msg_box
    }

}
/*

let task = stream.for_each(|item|{
            println!("{:?}", item);
            Ok(())
        }).map_err(|error|{
            println!("Yeeto dorrito there was an errorito! {}", error);
        });
        let spawned = tokio::spawn(task);

        let data : Vec<u8>;
        loop {
            match response {
                Async::Ready(item) => {
                    data = item.expect("Problem reading handshake response.");
                    break;
                }
                Async::NotReady => {
                    // Keep looping.
                },
            }
        }

        let received = bincode::deserialize::<NetMessageType>(&data[..]).unwrap();

        let mut player_id = 0;

        match received{
            NetMessageType::ConnectionInitResponse(response) => {
                println!("Successfully read handshake response!");
                player_id = response.assigned_player_id;
            },
            _ => {
                panic!("First item read from server after handshake request wasn't a handshake response!");
            },
        }


        let handshake_reponse = HandshakeResponse{
            player_id,
            socket_read: stream,
            socket_write: write_half,
        };
        Ok(())








*/

//        let meme = stream.poll();
//        let response = stream.poll().expect("Error reading handshake result.");







//let (tx_sender_handshake, rx_receiver_handshake): (Sender<NetMessageType>, Receiver<NetMessageType>) = mpsc::channel();
//let (tx_sender_normal, rx_receiver_normal): (Sender<NetMessageType>, Receiver<NetMessageType>) = mpsc::channel();
//
//let future = stream.for_each(move |stream_item|{
////            println!("Receiving: {:?}", stream_item);
//let received = bincode::deserialize::<NetMessageType>(&stream_item[..]).unwrap();
//match &received{
//NetMessageType::ConnectionInitResponse(response) => {
//let meme_cloned = response.clone();
//println!("Read something from the server {:?}", meme_cloned);
//
//tx_sender_handshake.send(received).unwrap();
//},
//_ => {
//tx_sender_normal.send(received).unwrap();
//},
//}
//
//return Ok(())
//}).map_err(|e|{
//println!("MemeSupremeError");
//});
//tokio::spawn(future);




//    let mut connection_dcwct = task_connection_future(target_ip);
////
////    let (mut read_half_dcwct, mut write_half_dcwct) = connection_dcwct.split();
////    let mut stream_dcwct = FramedRead::new(read_half_dcwct, dans_codec::Bytes);


//    let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
//    let r = runtime.block_on(stream.into_future());
//
//    if let Ok((v, _)) = r {
//        println!("{:?}", v);
//    }

//    stream.readv(); TODO: Investigate these two options.
//    stream.take();

//    let mut stream_iterator = stream.wait();
//    let meme = Iterator::next(&mut stream_iterator).unwrap();

//    let first_item_read : Vec<u8> = meme.unwrap(); // TODO: Test :) Not sure how well this will work.










