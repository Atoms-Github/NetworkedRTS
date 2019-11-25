


use std::collections::HashMap;

use crate::players::inputs::InputState;
use crate::ecs::world::*;
use crate::network::networking_message_types::NetMessageType;

use crate::systems::position::{PositionComp, secret_position_system};
use crate::systems::velocity::*;
use crate::systems::render::*;
use crate::systems::velocityWithInput::*;
use crate::systems::size::*;


use std::sync::{Arc, Mutex};
use tokio::io::ReadHalf;
use tokio::net::TcpStream;
use crate::network::dans_codec::Bytes;
use tokio::codec::FramedRead;

use futures::stream::*;
use futures::future::*;
use std::borrow::BorrowMut;
use ggez::graphics;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::io::{self, BufRead};
use std::sync::mpsc::channel;

use crate::network::networking_message_types::*;

use std::thread;
use serde::{Serialize, Deserialize};



pub type PlayerID = usize;
pub type FrameIndex = usize;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState{
    pub world: World,
    pub storages: Storages,
    pub frame_count: i32,

}

impl GameState{
    pub fn new() -> GameState{
        GameState{
            world: World::new(),
            storages: Storages::new(),
            frame_count: 0
        }
    }
    pub fn simulate_tick(&mut self, inputs_info: &InputsFrame, delta: f32){
        let mut pending = PendingEntities::new();

        secret_position_system(&self.world, &mut pending, &mut self.storages.position_s, &mut self.storages.velocity_s);
        secret_velocity_system(&self.world, &mut pending, &mut self.storages.position_s, &mut self.storages.velocity_s);
        secret_velocity_with_inputs_system(&self.world, &mut pending, &mut self.storages.velocity_s,
        &mut self.storages.velocityWithInput_s, inputs_info);

        self.world.update_entities(&mut self.storages, pending);
    }
}



pub struct InputsFrame{
    pub inputs: HashMap<PlayerID, InputState>
}
pub struct InputFramesStorage{
    pub frames: Vec<InputsFrame>
}



impl InputFramesStorage{
    pub fn new() -> InputFramesStorage{
        InputFramesStorage{
            frames: vec![]
        }
    }
    pub fn insert_frames(&mut self, player_id: PlayerID, starting_index: usize, input_states: &[InputState; 20]){
        self.blanks_up_to_index(starting_index + input_states.len());


        for (current_index, input_state) in input_states.iter().enumerate(){ // TODO - Use fancy vector clone section method.
            self.frames[current_index].inputs.insert(player_id, input_state.clone()); // TODO - Use moves instead of clone.
        }
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



pub struct MessageBox {
    pub items: Arc<Mutex<Vec<NetMessageType>>>,
}

impl MessageBox {
    pub fn spawn_tokio_task_message_box_fill(&self, connection_readable: FramedRead<ReadHalf<TcpStream>, Bytes>){
        let message_box_mutex = Arc::clone(&self.items); // However this works :)

        let tokio_task = connection_readable.for_each( move |data| {
            let deserialized = bincode::deserialize::<NetMessageType>(&data[..]).unwrap();
            {
                let mut mutex_lock= Mutex::lock(&message_box_mutex).unwrap();
                mutex_lock.push(deserialized);
                std::mem::drop(mutex_lock); // Just to doubley ensure lock is dropped.
            }
            Ok(())
        }).map_err(|error|{
            println!("Yeeto dorrito there was an errorito!  (While client was reading data) {}", error);
        });

        tokio::spawn(tokio_task);
    }
    pub fn spawn_thread_fill_from_receiver(&self, receiver: Receiver<NetMessageType>){
        let meme = receiver;
        let message_box_mutex = Arc::clone(&self.items); // However this works :)

        thread::spawn(move ||{
            let dream = meme;
            loop{
                let item = dream.recv();
                match item{
                    Ok(net_message) => {
                        {
                            let mut mutex_lock= Mutex::lock(&message_box_mutex).unwrap();
                            mutex_lock.push(net_message);
                            std::mem::drop(mutex_lock); // Just to doubley ensure lock is dropped.
                        }
                    },
                    Err(err) => {
                        panic!("Error initing filling message box from reciever. {}", err);
                    },
                }
            }
        });
    }
    pub fn new() -> MessageBox {
        MessageBox {
            items: Arc::new(Mutex::new(vec![]))
        }
    }
    pub fn spawn_thread_read_cmd_input(&self){
        let (sender, reciever) = channel::<NetMessageType>();
        thread::spawn(||{
            let sink = sender;
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                sink.send(NetMessageType::LocalCommand(LocalCommandInfo{
                    command: line.expect("Problem reading std:io input line.")
                })).unwrap();
            }
        });
        self.spawn_thread_fill_from_receiver(reciever);
    }
}


pub struct PlayerProperties{
    pub name : String,
    pub player_id: PlayerID
}
impl PlayerProperties{
    pub fn new(player_id: PlayerID) -> PlayerProperties{
        PlayerProperties{
            name : String::from("NamelessWonder"),
            player_id: player_id
        }
    }
}