
use crate::network::networking_structs::*;
use crate::network::networking_message_types::*;
use crate::players::inputs::*;

use crate::ecs::world::*;
use crate::ecs::system_macro::*;

use tokio::io::WriteHalf;

use futures::future::Future;
use std::net::SocketAddr;
use std::io::{BufReader, Write};
use tokio::io::{lines, write_all};
use futures::sync::mpsc;
use serde::*;

extern crate tokio_core;
extern crate tokio_io;
use self::tokio_io::AsyncRead;
use crate::game::server_networking::*;


use std::sync::{Mutex, Arc};
use std::time::Duration;
use std::thread;


use futures::Stream;

use tokio::codec::{FramedRead, FramedWrite};
use std::collections::HashMap;

use crate::network::*;
use tokio::net::TcpListener;
use core::borrow::BorrowMut;
use futures::sink::Sink;

use std::time::{SystemTime, UNIX_EPOCH};

struct ServerMainState {
    game_state_tail: GameState,
    all_frames: InputFramesStorage,
    client_handles: HashMap<PlayerID, ClientHandle>,
    reception_data: Arc<Mutex<ServerReceptionData>>
//    client_message_box: MessageBox,
}

struct ServerReceptionData{
    new_player_handles: Vec<(PlayerID, ClientHandle)>,
    next_player_id: PlayerID
}
impl ServerReceptionData{
    fn new() -> ServerReceptionData{
        ServerReceptionData{
            new_player_handles: vec![],
            next_player_id: 4
        }
    }
}



pub fn server_main(hosting_ip: &String){
    println!("Starting as server. Going to host on {}", hosting_ip);

    let addr = hosting_ip.to_string().parse::<SocketAddr>().unwrap();
    let socket = TcpListener::bind(&addr).expect("Unable to bind hosting address.");


    let main_state = ServerMainState{
        game_state_tail: GameState::new(),
        all_frames: InputFramesStorage::new(),
        client_handles: Default::default(),
        reception_data: Arc::new(Mutex::new(ServerReceptionData::new() ))
    };

    let arc_reception_data = Arc::clone(&main_state.reception_data);

    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            let mut locked_reception = arc_reception_data.lock().unwrap();
            let new_player_id = locked_reception.next_player_id;
            locked_reception.next_player_id += 1;

            println!("New player connected. PlayerID: {} Address: {:?}", new_player_id , socket.peer_addr());
            let (reader, writer) = socket.split();

//            let sink = FramedWrite::new(writer, dans_codec::Bytes);

            let stream = FramedRead::new(reader, dans_codec::Bytes);

            let client_handle = ClientHandle{
                write_channel: writer,
                message_box: MessageBox::new(),
//                properties: PlayerProperties::new(new_player_id)
            };
            client_handle.message_box.spawn_tokio_task_message_box_fill(stream);

            locked_reception.new_player_handles.push((new_player_id, client_handle));
            println!("Added new player ");
            Ok(())
        });
    println!("Hosting on {}", hosting_ip);
    thread::spawn(move ||{
        main_server_logic(main_state);
    });
    tokio::run(done);
    println!("Server finished.");
}

fn main_server_logic(mut main_state: ServerMainState){
    println!("ServerLogic!");
    loop{
        thread::sleep(std::time::Duration::from_millis(1000));
        { // To drop mutex handle when appropriate.
            {
                let mut mutex_lock = Mutex::lock(&main_state.reception_data).unwrap();
                let mut reception_data = &mut *mutex_lock; // TODO: this is a bit of a spicy meme, now isn't it?
                for (player_id, mut client_handle) in reception_data.new_player_handles.drain(..){
                    main_state.client_handles.insert(player_id, client_handle);
                }
            }



            for (player_id, client_handle) in &mut main_state.client_handles {
                for message in client_handle.message_box.items.lock().unwrap().drain(..) {
                    match &message{
                        NetMessageType::ConnectionInitQuery(response) => {
                            let time = SystemTime::now();

                            let state_to_send = main_state.game_state_tail.clone(); // TODO this shouldn't need to be cloned to be serialized.
                            let frames_partial = main_state.all_frames.get_frames_partial(state_to_send.frame_count + 1);
                            let response = NetMessageType::ConnectionInitResponse(NetMsgConnectionInitResponse{
                                assigned_player_id: *player_id,
                                frames_gathered_so_far: frames_partial,
                                game_state: state_to_send,
                                server_time_of_state: time
                            });
                            let bytes = bincode::serialize(&response).unwrap();
                            println!("Sent init message to client: {:?}", bytes);
                            client_handle.write_channel.write(&bytes[..]);
                        },
                        _ => {
                            println!("Not implemented this type of message.");
                        },
                    }

                    println!("Got a message from client: {:?}", message);
                }
            }

        }
    }
}











