
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


use futures::Stream;

use tokio::codec::{FramedRead, FramedWrite};
use std::collections::HashMap;

use crate::network::*;
use tokio::net::TcpListener;


struct ServerMainState {
    game_state_tail: GameState,
    all_frames: InputFramesStorage,
    player_handles: HashMap<PlayerID, ClientHandle>,
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
            next_player_id: 0
        }
    }
}


pub fn server_main(hosting_ip: &String){
    println!("Starting as server. Going to host on {}", hosting_ip);

    let addr = hosting_ip.to_string().parse::<SocketAddr>().unwrap();
    let socket = TcpListener::bind(&addr).expect("Unable to bind hosting address.");


    let mut main_state = ServerMainState{
        game_state_tail: GameState::new(),
        all_frames: InputFramesStorage::new(),
        player_handles: Default::default(),
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

            let sink = FramedWrite::new(writer, dans_codec::Bytes);
            let stream = FramedRead::new(reader, dans_codec::Bytes);

            let client_handle = ClientHandle{
                write_channel: sink,
                message_box: MessageBox::new()
            };
            client_handle.message_box.init_message_box_filling(stream);

            locked_reception.new_player_handles.push((new_player_id, client_handle));
            println!("Added new player ");
            Ok(())
        });
    println!("Hosting on {}", hosting_ip);
    tokio::run(done);


    main_state.game_state_tail.frame_count = 2;



}