


use std::collections::HashMap;

use crate::players::inputs::InputState;
use crate::ecs::world::*;
use crate::network::networking_message_types::NetMessageType;
use std::sync::{Arc, Mutex};
use tokio::io::ReadHalf;
use tokio::net::TcpStream;
use crate::network::dans_codec::Bytes;
use tokio::codec::FramedRead;

use futures::stream::*;
use futures::future::*;
use std::borrow::BorrowMut;

pub type PlayerID = usize;
pub type FrameIndex = usize;



pub struct GameState{
    pub world: World,
    pub storages: Storages,
    pub frame_count: i32,
}

pub struct InputsFrame{
    pub inputs: HashMap<PlayerID, InputState>
}
pub struct InputFramesStorage{
    pub frames: Vec<InputsFrame>
}



impl InputFramesStorage{
    pub fn insert_frames(player_id: PlayerID, starting_index: usize, data: [InputState, 20]){

    }
    pub fn blanks_up_to_index(&mut self, target_index: usize){
        let number_to_add = target_index - self.frames.len() + 1;
        if number_to_add > 0{
            for iteration_index in 0..number_to_add { // TODO - google off by one exception. I'm expecting lower to be inclusive and upper to be exclusive.
                self.frames.push(InputsFrame{
                    inputs: HashMap::new() //Default::default()
                });
            }
        }

    }
}

pub struct MessageBox{
    pub items: Arc<Mutex<Vec<NetMessageType>>>,
}

impl MessageBox{




    pub fn init_message_box_filling(&self, connection_readable: &mut FramedRead<ReadHalf<TcpStream>, Bytes>){ // TODO - investigate why a reference is good enough.
        let message_box_mutex = Arc::clone(&self.items); // However this works :)

        connection_readable.for_each( move |data| {
            let deserialized = bincode::deserialize::<NetMessageType>(&data[..]).unwrap();

            let nabbed = message_box_mutex; // Moves the arc in to lamda?
            {
                let mut mutex_lock= Mutex::lock(&nabbed).unwrap();
                mutex_lock.push(deserialized);

                std::mem::drop(mutex_lock);
            }
            Ok(())
        }).map_err(|error|{
            println!("Yeeto dorrito there was an errorito!  (While client was reading data) {}", error);
        });

        tokio::run(connection_readable);

    }
    pub fn new() -> MessageBox{
        MessageBox{
            items: Arc::new(Mutex::new(vec![]))
        }
    }
}


impl GameState{
    pub fn new() -> GameState{
        GameState{
            world: World::new(),
            storages: Storages::new(),
            frame_count: 0
        }
    }

    pub fn simulate_tick(inputs_info: &InputsFrame, delta: f32){

    }
}