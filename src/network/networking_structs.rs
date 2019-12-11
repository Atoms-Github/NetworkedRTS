


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

use std::iter::FromIterator;

use crate::network::networking_message_types::*;

use std::thread;
use serde::{Serialize, Deserialize};



pub type PlayerID = usize;
pub type FrameIndex = usize;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState{
    pub world: World,
    pub storages: Storages,
    pub frame_count: usize,

}

impl GameState{
    pub fn new() -> GameState{
        GameState{
            world: World::new(),
            storages: Storages::new(),
            frame_count: 0
        }
    }
    pub fn init_rts(&mut self){
        let mut pending = PendingEntities::new();

        let mut pending_entity_online_player = PendingEntity::new();
        pending_entity_online_player.add_component(PositionComp{ x: 0.0, y: 0.0 });
        pending_entity_online_player.add_component(VelocityComp{ x: 0.0, y: 0.5 });
        pending_entity_online_player.add_component(SizeComp{ x: 50.0, y: 50.0 });
        pending_entity_online_player.add_component(RenderComp{ hue: (0,150,100)});
        pending.create_entity(pending_entity_online_player);

        self.world.update_entities(&mut self.storages, pending);
    }
    pub fn add_player(&mut self, player_id: PlayerID){
        let mut pending = PendingEntities::new();

        let mut pending_entity_online_player = PendingEntity::new();
        pending_entity_online_player.add_component(PositionComp{ x: 0.0, y: 0.0 });
        pending_entity_online_player.add_component(VelocityComp{ x: 100.0, y: 0.0 });
        pending_entity_online_player.add_component(SizeComp{ x: 50.0, y: 50.0 });
        pending_entity_online_player.add_component(velocityWithInputComp{ owner_id: player_id });
        pending_entity_online_player.add_component(RenderComp{ hue: (255,150,150)});
        pending.create_entity(pending_entity_online_player);

        self.world.update_entities(&mut self.storages, pending);
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


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputsFrame{
    pub inputs: HashMap<PlayerID, InputState>
}
impl InputsFrame{
    pub fn new() -> InputsFrame{
        InputsFrame{
            inputs: Default::default()
        }
    }
}
#[derive(Serialize, Deserialize,Clone,  Debug)]
pub struct FramesStoragePartial{
    pub frames_section: Vec<InputsFrame>,
    pub start_index: usize
}
#[derive(Clone)]
pub struct InputFramesStorage{
    pub frames: Vec<InputsFrame>
}



impl InputFramesStorage{
    pub fn new() -> InputFramesStorage{
        InputFramesStorage{
            frames: vec![]
        }
    }
    pub fn add_player_default_inputs(&mut self, player_id: &PlayerID, joined_player_frame_index: usize){
        for index in 0..20 {
            let mut meme = self.frames.get_mut(joined_player_frame_index + index).unwrap();
            meme.inputs.insert(*player_id, InputState::new());
        }
    }
    pub fn get_frames_partial(&self, first_index: usize) -> FramesStoragePartial{
        let partial;
        if first_index < self.frames.len(){
            partial = Vec::from_iter(self.frames[first_index..].iter().cloned()); // Clone out slice.
        }else{
            partial = vec![];
        }


        FramesStoragePartial{
            frames_section: partial,
            start_index: first_index
        }
    }
    pub fn insert_frames_partial(&mut self, partial: FramesStoragePartial){
        // Panic on overwrite attempt.
        // TODO investigate RAM usage of filling with hundreds of blanks. Might need to also store frames vector start index.
        if self.frames.len() > partial.start_index{
            panic!("Tried to overwrite existing frames by inserting a partial frame.");
        }
        self.blanks_up_to_index(partial.start_index - 1);
        for (iter_index, item) in partial.frames_section.into_iter().enumerate(){
            self.frames.insert(partial.start_index + iter_index, item)
        }
    }
    pub fn insert_frames(&mut self, player_id: PlayerID, starting_index: usize, input_states: &[InputState; 20]){ // TODO probably could merge insert_frames and insert_partial.
        self.blanks_up_to_index(starting_index + input_states.len());


        for (current_index, input_state) in input_states.iter().enumerate(){ // TODO - Use fancy vector clone section method.
            self.frames[current_index].inputs.insert(player_id, input_state.clone()); // TODO - Use moves instead of clone.
        }
    }
    pub fn blanks_up_to_index(&mut self, target_index: usize){
//        println!("A: {} B: {} ", target_index, self.frames.len());
        let number_to_add = target_index as i32 - self.frames.len() as i32 + 1;
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
            println!("Recieved length: {}", data.len());
            // TODO: Should crash if can't serialize.

            let result = bincode::deserialize::<NetMessageType>(&data[..]);
            match result{
                Ok(e) => {
                    {
                        let mut mutex_lock= Mutex::lock(&message_box_mutex).unwrap();
//                        println!("Adding to message box: {:?}", e);
                        mutex_lock.push(e);

                        std::mem::drop(mutex_lock); // Just to doubley ensure lock is dropped.
                    }
                }
                Err(err) => {
                    // TODO: Should crash.
                }
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