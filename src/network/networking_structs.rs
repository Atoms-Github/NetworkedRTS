use std::collections::HashMap;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::thread;

use serde::{Deserialize, Serialize};

use crate::ecs::world::*;
use crate::network::networking_message_types::*;
use crate::network::networking_message_types::NetMessageType;
use crate::players::inputs::InputState;
use crate::gameplay::systems::position::{PositionComp, secret_position_system};
use crate::gameplay::systems::render::*;
use crate::gameplay::systems::size::*;
use crate::gameplay::systems::velocity::*;
use crate::gameplay::systems::velocity_with_input::*;
use crate::players::inputs::*;
use std::panic;

use crate::game::bonus_msgs_segment::*;

pub type PlayerID = usize;
pub type FrameIndex = usize;



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState{
    pub world: World,
    pub storages: Storages,
    /* Private */simmed_frame_index: FrameIndex,
}

impl GameState{
    pub fn get_simmed_frame_index(&self) -> FrameIndex{
        return self.simmed_frame_index;
    }
    pub fn new() -> GameState{
        GameState{
            world: World::new(),
            storages: Storages::new(),
            simmed_frame_index: 0
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
    pub fn init_new_player(&mut self, player_id: PlayerID){
        let mut pending = PendingEntities::new();

        let mut pending_entity_online_player = PendingEntity::new();
        pending_entity_online_player.add_component(PositionComp{ x: 0.0, y: 0.0 });
        pending_entity_online_player.add_component(VelocityComp{ x: 1.0, y: 0.0 });
        pending_entity_online_player.add_component(SizeComp{ x: 50.0, y: 50.0 });
        pending_entity_online_player.add_component(VelocityWithInputComp { owner_id: player_id });
        pending_entity_online_player.add_component(RenderComp{ hue: (255,150,150)});
        pending.create_entity(pending_entity_online_player);

        self.world.update_entities(&mut self.storages, pending);
    }
    fn apply_bonus_event(&mut self, bonus_event: BonusEvent){
        match bonus_event{
            BonusEvent::NewPlayer(player_id) => {
                self.init_new_player(player_id);
            }
        }
    }
    pub fn simulate_tick(&mut self, sim_info: InfoForSim, delta: f64){
        for bonus_event in sim_info.bonus_events{
            self.apply_bonus_event(bonus_event);
        }
        let mut pending = PendingEntities::new();

        secret_position_system(&self.world, &mut pending, &mut self.storages.position_s, &mut self.storages.velocity_s);
        secret_velocity_system(&self.world, &mut pending, &mut self.storages.position_s, &mut self.storages.velocity_s);
        secret_velocity_with_inputs_system(&self.world, &mut pending, &mut self.storages.velocity_s,
        &mut self.storages.velocity_with_input_s, &sim_info.inputs_map);

        self.world.update_entities(&mut self.storages, pending);

        self.simmed_frame_index = self.simmed_frame_index + 1;
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InfoForSim {
    pub inputs_map: HashMap<PlayerID, InputState>,
    pub bonus_events: Vec<BonusEvent>
}


//#[derive(Clone, Serialize, Deserialize, Debug)]
//pub struct PlayerInputsRecord {
//    pub inputs: Vec<InputState>,
//    pub start_frame: FrameIndex
//}
//impl PlayerInputsRecord {
//    pub fn new(start_frame: FrameIndex) -> PlayerInputsRecord {
//        PlayerInputsRecord {
//            inputs: Default::default(),
//            start_frame
//        }
//    }
//    pub fn get_input_frame_abs(&self, target_frame_abs: &FrameIndex) -> Option<&InputState>{
//        let relative_frame = target_frame_abs - self.start_frame;
//        return self.inputs.get(relative_frame);
//    }
//}
//#[derive(Serialize, Deserialize,Clone,  Debug)]
//pub struct FramesStoragePartial{
//    pub frames_section: Vec<PlayerInputsRecord>,
//    pub start_index: usize
//}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlayerInputSegmentType{
    Change(InputChange),
    WholeState(InputState)
}





fn blanks_to_frame(vector: &mut Vec<InputState>, relative_frame_index: FrameIndex){ // TODO3: Move somewhere.
    for index in vector.len()..(relative_frame_index+1) /*Start exclusive, end inclusive.*/{
        vector.push(InputState::new()); // Fill in with blanks.
    }
}





pub struct MessageBox {
    pub items: Arc<Mutex<Vec<NetMessageType>>>,
}

impl MessageBox {
//    pub fn spawn_thread_message_box_fill(&self, connection_readable: TcpStream){
//        let message_box_mutex = Arc::clone(&self.items); // However this works :)
//
//        thread::spawn(move ||{
//            let inc_messages = start_inwards_codec_thread(connection_readable);
//
//            loop{
//                let message = inc_messages.recv().unwrap();
//
//                {
//                    let mut mutex_lock= Mutex::lock(&message_box_mutex).unwrap();
//                    println!("Adding to message box: {:?}", e);
//                    mutex_lock.push(message);
//                }
//
//            }
//        });
//    }
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
            player_id
        }
    }
}