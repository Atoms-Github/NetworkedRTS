use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::thread;

use serde::{Deserialize, Serialize};

use crate::ecs::world::*;
use crate::network::networking_message_types::*;
use crate::network::networking_message_types::NetMessageType;
use crate::players::inputs::InputState;
use crate::systems::position::{PositionComp, secret_position_system};
use crate::systems::render::*;
use crate::systems::size::*;
use crate::systems::velocity::*;
use crate::systems::velocity_with_input::*;
use crate::network::game_message_types::*;
use crate::players::inputs::*;
use std::panic;
use crate::utils::util_functions::vec_replace_or_end;
use crate::network::game_message_types::NewPlayerInfo;
use nalgebra::abs;

pub type PlayerID = usize;
pub type FrameIndex = usize;



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState{
    pub world: World,
    pub storages: Storages,
}

impl GameState{
    pub fn new() -> GameState{
        GameState{
            world: World::new(),
            storages: Storages::new()
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
        pending_entity_online_player.add_component(VelocityWithInputComp { owner_id: player_id });
        pending_entity_online_player.add_component(RenderComp{ hue: (255,150,150)});
        pending.create_entity(pending_entity_online_player);

        self.world.update_entities(&mut self.storages, pending);
    }
    pub fn simulate_tick(&mut self, inputs_info: &InfoForSim, delta: f64){
        let mut pending = PendingEntities::new();

        secret_position_system(&self.world, &mut pending, &mut self.storages.position_s, &mut self.storages.velocity_s);
        secret_velocity_system(&self.world, &mut pending, &mut self.storages.position_s, &mut self.storages.velocity_s);
        secret_velocity_with_inputs_system(&self.world, &mut pending, &mut self.storages.velocity_s,
        &mut self.storages.velocity_with_input_s, inputs_info);

        self.world.update_entities(&mut self.storages, pending);
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InfoForSim {
    pub inputs_map: HashMap<PlayerID, InputState>,
    pub bonus_events: Vec<BonusEvent>
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlayerInputsRecord {
    pub inputs: Vec<InputState>,
    pub start_frame: FrameIndex
}
impl PlayerInputsRecord {
    pub fn new(start_frame: FrameIndex) -> PlayerInputsRecord {
        PlayerInputsRecord {
            inputs: Default::default(),
            start_frame
        }
    }
    pub fn get_input_frame_abs(&self, target_frame_abs: &FrameIndex) -> Option<&InputState>{
        let relative_frame = target_frame_abs - self.start_frame;
        return self.inputs.get(relative_frame);
    }
}
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BonusEvent{
    NewPlayer(PlayerID),
    None
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputFramesStorage{
    pub frames_map: HashMap<PlayerID, PlayerInputsRecord>,
    pub bonus_events: Vec<Vec<BonusEvent>>,
    pub bonus_start_frame: FrameIndex,
}

fn blanks_to_frame(vector: &mut Vec<InputState>, relative_frame_index: FrameIndex){ // TODO3: Move somewhere.
    for index in vector.len()..(relative_frame_index+1) /*Start exclusive, end inclusive.*/{
        vector.push(InputState::new()); // Fill in with blanks.
    }
}

impl InputFramesStorage{
    pub fn new(start_frame: FrameIndex) -> InputFramesStorage{
        InputFramesStorage{
            frames_map: Default::default(),
            bonus_events: vec![],
            bonus_start_frame: start_frame
        }
    }
    pub fn add_player(&mut self, new_player_info: &NewPlayerInfo){
        self.frames_map.insert(new_player_info.player_id, PlayerInputsRecord::new(new_player_info.frame_added));
    }
    pub fn get_simable_info(){

    }
    pub fn calculate_last_inputs(&self) -> HashMap<PlayerID, InputState>{
        let mut to_return = HashMap::new();

        for (player_id,player_record) in self.frames_map.iter(){
            let last_input= player_record.inputs.last();
            let usable_input;
            match last_input{
                Some(state) => {
                    usable_input = state.clone();
                }
                None => {
                    usable_input = InputState::new();
                }

            }
            to_return.insert(*player_id, usable_input);
        }

        return to_return;
    }
    pub fn get_frames_segment(&self, segment_needed: &LogicInfoRequest) -> Option<LogicInwardsMessage> {
        match segment_needed.type_needed{
            LogicInfoRequestType::PlayerInputs(player_id) => {
                // Eventually..., this whole thing can probably be sped up by not cloning anywhere. Just using fancy lifetimed references.
                let player_record = self.frames_map.get(&player_id)?; // Wayyyy, using question marks like a boss. :)
                let relative_start_frame = segment_needed.start_frame - player_record.start_frame;


                let mut input_states_found = vec![];
                for relative_index in relative_start_frame..(relative_start_frame + segment_needed.number_of_frames /*No need for +1 */){
                    let inputs = player_record.inputs.get(relative_index);
                    if inputs.is_some(){
                        let input_segment = PlayerInputSegmentType::WholeState(inputs.unwrap().clone());
                        input_states_found.push(input_segment);
                    }

                }

                return Some(LogicInwardsMessage::InputsUpdate(LogicInputsResponse{
                    player_id,
                    start_frame_index: segment_needed.start_frame,
                    input_states: input_states_found
                }));
            }
            LogicInfoRequestType::BonusEvents => {
                // This should never be called on the client.
                let mut events = vec![];
                for abs_index in segment_needed.start_frame..(segment_needed.start_frame + segment_needed.number_of_frames){
                    let relative_index = abs_index - self.bonus_start_frame;
                    let events_list = self.bonus_events.get(abs_index);
                    if events_list.is_some(){
                       events.push(events_list.unwrap().clone());
                    }else{
                        break; // Reached end of list.
                    }
                }
                let msg = LogicInwardsMessage::BonusMsgsUpdate(BonusMsgsResponse{
                    start_frame_index: segment_needed.start_frame,
                    event_lists: events
                });
                return Some(msg);
            }
        }

    }
    pub fn insert_bonus_segment(&mut self, segment: &BonusMsgsResponse){

        for (source_rel_index, events) in segment.event_lists.iter().enumerate(){
            let abs_index = source_rel_index + segment.start_frame_index;
            let target_rel_index = abs_index - self.bonus_start_frame;
            vec_replace_or_end(&mut self.bonus_events, target_rel_index, events.clone()); // Pointless_optimum Clone.
        }
    }

    pub fn insert_frames_segment(&mut self, segment: &LogicInputsResponse){
//        let map = self.frames_map.get_mut(&segment.player_id).expect("Tried to insert frames for player that wasn't stored.");
        for (input_vec_index, item) in segment.input_states.iter().enumerate(){
            let absolute_index = segment.start_frame_index + input_vec_index;



            // I know its inneficient, and can be replaced by this: https://stackoverflow.com/questions/28678615/efficiently-insert-or-replace-multiple-elements-in-the-middle-or-at-the-beginnin
            // But the other solution will get a bit complicated as the end of the slice to insert can be off the end of the vector.
            let map = self.frames_map.get_mut(&segment.player_id).expect("Tried to insert frames for player that wasn't stored.");
            let relative_index = absolute_index - map.start_frame;

            match item.clone(){
                // TODO3: Optimise this whole section.
                PlayerInputSegmentType::WholeState(state) => {

                    vec_replace_or_end(&mut map.inputs, relative_index, state);
                }
                PlayerInputSegmentType::Change(input_change) => {
                    blanks_to_frame(&mut map.inputs, absolute_index);
                    let state = map.inputs.get_mut(relative_index).unwrap();
                    input_change.apply_to_state(state);
                }
            }

        }
    }
//    pub fn blanks_up_to_index(&mut self, target_index: usize){
////        println!("A: {} B: {} ", target_index, self.frames.len());
//        let number_to_add = target_index as i32 - self.frames.len() as i32 + 1;
//        if number_to_add > 0{
//            for iteration_index in 0..number_to_add {
//                self.frames.push(PlayerInputsRecord {
//                    inputs: HashMap::new() //Default::default()
//                });
//            }
//        }
//    }
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