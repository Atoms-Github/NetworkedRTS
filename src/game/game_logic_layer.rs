use std::sync::mpsc::{Receiver, TryRecvError};
use crate::network::game_message_types::GameMessageType;
use std::thread;
use crate::players::inputs::KnownFrameInfo;
use crate::network::networking_structs::*;
use crate::network::networking_message_types::*;
use crate::players::inputs::*;
use crate::game::client_networking::connect_and_send_handshake;
use crate::systems::render::*;
use crate::ecs::world::*;
use crate::ecs::system_macro::*;
use crate::network::*;
use std::sync::{Mutex, Arc};


pub struct GameLogicLayer{
    does_update_head: bool,
    known_frame_info: KnownFrameInfo,
    game_state_head: Arc<Mutex<GameState>>,
    game_state_tail: GameState,
    all_frames: InputFramesStorage,
    // Logic layer shouldn't know it's player ID.
}


impl GameLogicLayer{
    pub fn new(update_head :bool, known_frame_info: KnownFrameInfo, state_tail: GameState) -> (GameLogicLayer, Arc<Mutex<GameState>>){
        let game_state_head = Arc::new(Mutex::new(state_tail.clone()));
        (
        GameLogicLayer{
            does_update_head: update_head,
            known_frame_info,
            game_state_head,
            game_state_tail: state_tail,
            all_frames: InputFramesStorage::new(),
        },
        game_state_head.clone())

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
                self.game_state_tail.add_player(new_player_info.player_id);
                self.all_frames.add_player_default_inputs(&new_player_info.player_id, new_player_info.frame_added)
            }
        }
    }
    fn update_tail(&mut self, target_frame_tail: usize){
        while self.game_state_tail.last_frame_simed < target_frame_tail{
            self.game_state_tail.last_frame_simed += 1;

            let frame_index_to_simulate = self.game_state_tail.last_frame_simed;

            let inputs_to_use = self.all_frames.frames.get(frame_index_to_simulate).expect("Panic! Required frames haven't arrived yet. OH MY HOMIES!");
            self.game_state_tail.simulate_tick(inputs_to_use, 0.016 /* TODO: Use real delta. */);

        }
    }
    fn update_head(&mut self, target_frame_head: usize){
        let mut head_to_be = self.game_state_tail.clone();
        while head_to_be.last_frame_simed < target_frame_head{
            head_to_be.last_frame_simed += 1;

            let frame_index_to_simulate = head_to_be.last_frame_simed;
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
    pub fn run_logic_loop(&mut self, mut inputs_channel: Receiver<GameMessageType>){
        loop{
            self.apply_available_game_messages(&mut inputs_channel);

            let target_frame_tail = self.known_frame_info.get_intended_current_frame();
            let target_frame_head = target_frame_tail + 19;

            self.all_frames.blanks_up_to_index(target_frame_head); // TODO: Should detect and handle when inputs don't come in.

            self.update_tail(target_frame_tail);

            if self.does_update_head { // Don't bother on the server.
                self.update_head(target_frame_head);
            }
        }
    }
}
