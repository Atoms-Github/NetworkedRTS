use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, TryRecvError};

use crate::game::timekeeping::*;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::game_message_types::{GameMessageType, NewPlayerInfo};
use crate::network::networking_structs::*;
use std::panic;

pub const HEAD_FRAME_LEAD : usize = 19;


pub struct LogicSegment {
    head_is_ahead: bool,
    known_frame_info: KnownFrameInfo,
    game_state_head: Arc<Mutex<GameState>>,
    game_state_tail: GameState,
    all_frames: InputFramesStorage,
    // Logic layer shouldn't know it's player ID.
}


impl LogicSegment {
    pub fn new(head_is_ahead:bool, known_frame_info: KnownFrameInfo, state_tail: GameState) -> (LogicSegment, Arc<Mutex<GameState>>){
        let game_state_head = Arc::new(Mutex::new(state_tail.clone()));
        (
            LogicSegment {
            head_is_ahead,
            known_frame_info,
            game_state_head: game_state_head.clone(),
            game_state_tail: state_tail,
            all_frames: InputFramesStorage::new(),
        },
            game_state_head)

    }
    fn apply_available_game_messages(&mut self, inputs_channel: &mut Receiver<GameMessageType>){
        loop{
            let game_message = inputs_channel.try_recv();
            match game_message{
                Ok(item) => {
                    self.apply_game_message(&item);
                }
                Err(err) => {
                    if err == TryRecvError::Disconnected{
                        panic!("Input stream disconnected.");
                    }
                    break;
                }
            }
        }

    }
    fn apply_game_message(&mut self, message: &GameMessageType){
        match message{
            GameMessageType::InputsUpdate(inputs_update) => {
                self.all_frames.insert_frames(inputs_update.player_id,inputs_update.frame_index, &inputs_update.input_states);
            }
            GameMessageType::NewPlayer(new_player_info) => {
                self.add_new_player(new_player_info);

            }
        }
    }
    pub fn add_new_player(&mut self, new_player_info: &NewPlayerInfo){
        self.game_state_tail.add_player(new_player_info.player_id);
        self.all_frames.add_player_default_inputs(&new_player_info.player_id, new_player_info.frame_added)
    }
    fn sim_tail_frame(&mut self, tail_frame_to_sim: FrameIndex){
        self.game_state_tail.last_frame_simed = tail_frame_to_sim;
        let inputs_to_use = self.all_frames.frames.get(tail_frame_to_sim).expect("Panic! Required frames haven't arrived yet. OH MY HOMIES!");
        self.game_state_tail.simulate_tick(inputs_to_use, FRAME_DURATION);
    }
    fn resimulate_head(&mut self, tail_frame: FrameIndex){
        let mut head_to_be = self.game_state_tail.clone();
        for frame_index_to_simulate in tail_frame..(tail_frame + HEAD_FRAME_LEAD + 1){
            head_to_be.last_frame_simed += 1; // TODO: Shouldn't be needed if this field is removed. Can be done better.

//                println!("Simulating frame nubmer {}", frame_index_to_simulate);

            let possible_arrived_inputs = self.all_frames.frames.get(frame_index_to_simulate);
            let inputs_to_use;
            let blank_inputs = InputsFrame::new();
            match possible_arrived_inputs{
                Some(inputs) => {
                    inputs_to_use = inputs;
                }
                None=> {
                    inputs_to_use = &blank_inputs; // TODO: Should 1. use the last known input, not nothing. And 2. should split inputs by players, so only unknown players are guessed.
                }
            }
            head_to_be.simulate_tick(inputs_to_use, 0.016 /* TODO: Use real delta. */);
        }
        {
            *self.game_state_head.lock().unwrap() = head_to_be; // Update mutex lock.
        }
    }
    fn set_head_to_tail(&mut self){
        let mut meme = self.game_state_head.lock().unwrap();
        *meme = self.game_state_tail.clone();

    }
    pub fn load_frames(&mut self, frames_partial: FramesStoragePartial){
        self.all_frames.insert_frames_partial(frames_partial);
    }


    pub fn run_logic_loop(mut self, mut game_messages_channel: Receiver<GameMessageType>){
        let mut generator = self.known_frame_info.start_frame_stream();
        loop{
            let tail_frame_to_sim = generator.recv().unwrap();

            self.apply_available_game_messages(&mut game_messages_channel);

            self.all_frames.blanks_up_to_index(tail_frame_to_sim + HEAD_FRAME_LEAD); // TODO: Should detect and handle when inputs don't come in.

            self.sim_tail_frame(tail_frame_to_sim);

            if self.head_is_ahead { // Don't bother on the server.
                self.resimulate_head(tail_frame_to_sim);
            }else{
                self.set_head_to_tail();
            }
        }
    }
}



//let controllers = self.get_controllers_clone();
//
//let mut pending = PendingEntities::new();
//
//secret_position_system(&self.world, &mut pending, &mut self.storage.position_s, &mut self.storage.velocity_s);
//secret_velocity_system(&self.world, &mut pending, &mut self.storage.position_s, &mut self.storage.velocity_s);
//secret_velocityWithInput_system(&self.world, &mut pending, &mut self.storage.velocity_s,
//&mut self.storage.velocityWithInput_s, &controllers);
//
//self.world.update_entities(&mut self.storage, pending);



//
////            std::mem::drop();
//{ // Need to be explicit in where the mutex locks are dropped.
//let mut messages_this_frame = self.messages_to_process.lock().unwrap();
//for net_message in &*messages_this_frame{
//match net_message{
//NetMessageType::ConnectionInit(msg_init) => {
//println!("Welcomed with a message: {}", msg_init.welcome_msg);
//self.online_players.push(OnlinePlayer{
//controller: PlayerController { input_state: InputState::new()}
//});
//
//
//let mut pending = PendingEntities::new();
//
//
//let mut pending_entity_online_player = PendingEntity::new();
//pending_entity_online_player.add_component(PositionComp{ x: 0.0, y: 0.0 });
//pending_entity_online_player.add_component(VelocityComp{ x: 2.0, y: 1.0 });
//pending_entity_online_player.add_component(SizeComp{ x: 50.0, y: 50.0 });
//pending_entity_online_player.add_component(velocityWithInputComp{ owner_id: 2 });
//pending_entity_online_player.add_component(RenderComp{ hue: graphics::Color::from_rgb(255,150,150) });
//pending.create_entity(pending_entity_online_player);
//
//
//
//self.world.update_entities(&mut self.storage, pending);
//
//
//
//},
//NetMessageType::InputsUpdate(msg_inputs) => {
//// TODO - need to do some player ID matching here.
//for online_player in &mut self.online_players{
//online_player.controller = msg_inputs.controllers[0].clone();
//}
//
//},
//};
//}
//messages_this_frame.clear();
//}
