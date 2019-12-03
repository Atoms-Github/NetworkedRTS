
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
    reception_data: Arc<Mutex<ServerReceptionData>>,
    big_fat_zero_time: KnownFrameInfo
//    client_message_box: MessageBox,
}

pub fn server_main(hosting_ip: &String){
    println!("Starting as server. Going to host on {}", hosting_ip);

    let addr = hosting_ip.to_string().parse::<SocketAddr>().unwrap();
    let socket = TcpListener::bind(&addr).expect("Unable to bind hosting address.");


    let mut main_state = ServerMainState{
        game_state_tail: GameState::new(),
        all_frames: InputFramesStorage::new(),
        client_handles: Default::default(),
        reception_data: Arc::new(Mutex::new(ServerReceptionData::new() )),
        big_fat_zero_time: KnownFrameInfo{
            frame_index: 0,
            time: SystemTime::now()
        }
    };
    main_state.game_state_tail.init_rts();

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
        main_state.main_server_logic();
    });
    tokio::run(done);
    println!("Server finished.");
}

impl ServerMainState{
    fn add_client_handles(&mut self) -> Vec<PlayerID>{
        let mut new_player_ids = vec![];
        {
            let mut mutex_lock = Mutex::lock(&self.reception_data).unwrap();
            let mut reception_data = &mut *mutex_lock; // TODO: this is a bit of a spicy meme, now isn't it?
            for (player_id, mut client_handle) in reception_data.new_player_handles.drain(..){
                self.client_handles.insert(player_id, client_handle);
                new_player_ids.push(player_id);
            }
        }
        return new_player_ids;
    }
    fn main_server_logic(mut self){
        println!("ServerLogic!");
        loop{
            thread::sleep(std::time::Duration::from_millis(1000));
            println!("Server frame collection size: {}", self.all_frames.frames.len());

            // Fill with blanks if player's don't do anything in order for new players to recieve some input log to prevent oh my homies.
            self.all_frames.blanks_up_to_index(self.big_fat_zero_time.get_intended_current_frame() + 20); // TODO: Should detect and handle when inputs don't come in.
            let new_player_ids = self.add_client_handles();

            for (player_id, client_handle) in &mut self.client_handles {
                for new_player_id in &new_player_ids{
                    if *player_id == *new_player_id{ // Don't tell the player that they themselves have been added.
                        continue;
                    }
                    let new_player_msg = NetMessageType::NewPlayer(NetMsgNewPlayer{
                        player_id: *new_player_id,
                        frame_added: self.game_state_tail.frame_count // TODO: Make sure simulation's current frame number is synced.
                    });
                    let new_player_msg_bytes = bincode::serialize(&new_player_msg).unwrap();
                    client_handle.write_channel.write(&new_player_msg_bytes[..]);
                }
            }

            for new_player_id in &new_player_ids {
                self.game_state_tail.add_player(*new_player_id);
                self.all_frames.add_player_default_inputs(new_player_id, self.game_state_tail.frame_count);
            }

            for (player_id, client_handle) in &mut self.client_handles {
                for message in client_handle.message_box.items.lock().unwrap().drain(..) {
                    match &message{

                        NetMessageType::ConnectionInitQuery(response) => {
                            let time = SystemTime::now();

                            let state_to_send = self.game_state_tail.clone(); // TODO this shouldn't need to be cloned to be serialized.
                            let frames_partial = self.all_frames.get_frames_partial(state_to_send.frame_count + 1);
                            let response = NetMessageType::ConnectionInitResponse(NetMsgConnectionInitResponse{
                                assigned_player_id: *player_id,
                                frames_gathered_so_far: frames_partial,
                                known_frame_info: KnownFrameInfo { frame_index: state_to_send.frame_count, time },
                                game_state: state_to_send,
                            });
                            let bytes = bincode::serialize(&response).unwrap();

                            println!("Sending init message to client: {:?} {:?}", bytes, response);
                            println!("Init message bytes size: {}", bytes.len());
                            client_handle.write_channel.write(&bytes[..]);
                        },
                        other => {
                            println!("Not implemented this type of message. {:?}", other);
                        },
                    }

                    println!("Got a message from client: {:?}", message);
                }
            }


            // TODO: simulate server tick somewhere near here, and make sure its current frame number is synced with the player joined frame index sent to clients.
        }
    }
}





// TODO: Look into using custom NetMsg codex instead of explicit de/serialization.






