use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, TryRecvError, Sender};

use crate::game::timekeeping::*;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::game_message_types::{LogicInwardsMessage, NewPlayerInfo, LogicOutwardsMessage};
use crate::network::networking_structs::*;
use crate::network::game_message_types::*;
use std::panic;
use std::collections::HashMap;
use std::thread::Thread;
use std::time::Duration;


pub const HEAD_AHEAD_FRAME_COUNT: usize = 20;


type Meme<T> = Arc<Mutex<T>>;

pub struct LogicSegment {
    head_is_ahead: bool,
    known_frame_info: KnownFrameInfo,
    game_state_head: Arc<Mutex<GameState>>,
    game_state_tail: GameState,
    all_frames: InputFramesStorage,
    outwards_messages: Sender<LogicOutwardsMessage>
    // Logic layer shouldn't know it's player ID.
}


impl LogicSegment {
    pub fn new(head_is_ahead:bool, known_frame_info: KnownFrameInfo, state_tail: GameState, outwards_messages: Sender<LogicOutwardsMessage>)
               -> (LogicSegment, Arc<Mutex<GameState>>){
        let game_state_head = Arc::new(Mutex::new(state_tail.clone()));
        let bonus_events_zero = known_frame_info.get_intended_current_frame();
        (
            LogicSegment {
            head_is_ahead,
            known_frame_info,
            game_state_head: game_state_head.clone(),
            game_state_tail: state_tail,
            all_frames: InputFramesStorage::new(bonus_events_zero),
                outwards_messages
            },
            game_state_head)
    }
    fn apply_available_game_messages(&mut self, inputs_channel: &mut Receiver<LogicInwardsMessage>){
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
    fn apply_game_message(&mut self, message: &LogicInwardsMessage){
        match message{
            LogicInwardsMessage::InputsUpdate(inputs_update) => {
                self.all_frames.insert_frames_segment(inputs_update);
            }
            LogicInwardsMessage::BonusMsgsUpdate(bonus_msg_response) => {

            }
        }
    }
    fn resimulate_head(&mut self, tail_frame_just_simed: FrameIndex){
        let mut head_to_be = self.game_state_tail.clone();
        let first_head_to_sim = tail_frame_just_simed + 1;
        // Pointless_optimum: Could probably be sped with references instead of cloning.
        let mut players_last_input = self.all_frames.calculate_last_inputs();

        for frame_index_to_simulate in first_head_to_sim..(first_head_to_sim + HEAD_AHEAD_FRAME_COUNT){
            let mut inputs_to_sim_with = HashMap::new();
            for (player_id,player_record) in self.all_frames.frames_map.iter(){
                let player_inputs = player_record.get_input_frame_abs(&frame_index_to_simulate);
                match player_inputs{
                    Some(inputs) => {
                        inputs_to_sim_with.insert(*player_id, inputs.clone());
                    },
                    None => {
                        inputs_to_sim_with.insert(*player_id, players_last_input.get(player_id).unwrap().clone());
                    }
                }
            }

            let bonus_relative_frame_index = frame_index_to_simulate - self.all_frames.bonus_start_frame;
            let bonus_infos = self.all_frames.bonus_events.get(bonus_relative_frame_index);
            let bonus_infos_to_use;
            if bonus_infos.is_some(){
                bonus_infos_to_use = bonus_infos.unwrap().clone();
            }else{
                bonus_infos_to_use = vec![];
            }


            let sim_info = InfoForSim{
                inputs_map: inputs_to_sim_with,
                bonus_events: bonus_infos_to_use
            };
            head_to_be.simulate_tick(&sim_info, 0.016 /* TODO2: Use real delta. */);
        }
        {
            *self.game_state_head.lock().unwrap() = head_to_be; // Update mutex lock.
        }
    }
    fn sim_tail_frame(&mut self, tail_frame_to_sim: FrameIndex) -> Option<LogicInfoRequest>{
        let mut all_inputs = HashMap::new();
        for (player_id,player_record) in self.all_frames.frames_map.iter(){
            let player_inputs = player_record.get_input_frame_abs(&tail_frame_to_sim);
            match player_inputs{
                Some(inputs) => {
                    all_inputs.insert(*player_id, inputs.clone());
                },
                None => {
                    let missing_inputs_request = LogicInfoRequest {
                        start_frame: tail_frame_to_sim,
                        number_of_frames: 20, // Can be any.
                        type_needed: LogicInfoRequestType::PlayerInputs(*player_id),
                    };
                    return Some(missing_inputs_request);
                }
            }
        }
        let bonus_relative_frame_index = tail_frame_to_sim - self.all_frames.bonus_start_frame;
        let bonus_infos = self.all_frames.bonus_events.get(bonus_relative_frame_index);
        if bonus_infos.is_none(){
            return Some(LogicInfoRequest{
                start_frame: tail_frame_to_sim,
                number_of_frames: 20,
                type_needed: LogicInfoRequestType::BonusEvents
            });
        }
        let sim_info = InfoForSim{
            inputs_map: all_inputs,
            bonus_events: bonus_infos.unwrap().clone(),
        };
        self.game_state_tail.simulate_tick(&sim_info, FRAME_DURATION_MILLIS);

        return None; // No missing frames.
    }
    fn set_head_to_tail(&mut self){
        let mut meme = self.game_state_head.lock().unwrap();
        *meme = self.game_state_tail.clone();

    }

    pub fn load_frames(&mut self, storage: InputFramesStorage){
        self.all_frames = storage;
    }
    pub fn run_logic_loop(mut self, mut game_messages_channel: Receiver<LogicInwardsMessage>){
        let mut generator = self.known_frame_info.start_frame_stream();
        loop{
            let tail_frame_to_sim = generator.recv().unwrap();

            loop{ // Wait until inputs have arrived so tail can be simulated.
                self.apply_available_game_messages(&mut game_messages_channel);
                match self.sim_tail_frame(tail_frame_to_sim){
                    None => {
                        break; // Inputs have arrived.
                    },
                    Some(request) => {
                        println!("Logic missing info so asking: {:?}", request);
                        self.outwards_messages.send( LogicOutwardsMessage::InputsNeeded(request) ).unwrap();
                        std::thread::sleep(Duration::from_millis(100)); // Wait to save CPU cycles. modival: Can optimise recovery time by increasing check rate, but resend rate shouldn't be too high cos can't have too many messages.
                    }
                }
            }
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
//for online_player in &mut self.online_players{
//online_player.controller = msg_inputs.controllers[0].clone();
//}
//
//},
//};
//}
//messages_this_frame.clear();
//}
