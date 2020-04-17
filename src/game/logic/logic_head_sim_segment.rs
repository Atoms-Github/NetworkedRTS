use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, TryRecvError, Sender};
use serde::{Deserialize, Serialize};

use crate::game::timekeeping::*;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_structs::*;
use std::{panic, thread};
use std::collections::HashMap;
use std::time::Duration;
use crate::game::synced_data_stream::*;
use crate::players::inputs::*;
use crate::game::bonus_msgs_segment::*;
use crate::game::logic::logic_data_storage::*;

pub const HEAD_AHEAD_FRAME_COUNT: usize = 20;


pub struct LogicHeadSim {
    known_frame_info: KnownFrameInfo,
    game_state_head: Arc<RwLock<GameState>>,
    game_state_tail: Arc<RwLock<GameState>>,
    all_frames: Arc<RwLock<LogicDataStorage>>,
//    outwards_messages: Sender<LogicOutwardsMessage>
}

struct InfoForHeadSim {
    player_last_inputs: HashMap<PlayerID, InputState>,
    player_inputs: [],
    bonus_events: ,
}


impl LogicHeadSim {
    pub fn new(known_frame_info: KnownFrameInfo, state_tail: GameState,
               outwards_messages: Sender<LogicOutwardsMessage>, data_store: Arc<RwLock<LogicDataStorage>>) // TODO2: Refactor arguments.
               -> (LogicSegment, Arc<Mutex<GameState>>){
        let game_state_head = Arc::new(Mutex::new(state_tail.clone()));
        (
            LogicHeadSim {
                known_frame_info,
                game_state_head: game_state_head.clone(),
                game_state_tail: state_tail,
                all_frames: data_store
            },
            game_state_head)
    }
    pub fn start(mut self) -> Arc<RwLock<GameState>>{
        let head_handle = self.game_state_head.clone();
        thread::spawn(move ||{
            let my_self = self;

            loop{
                let tail = self.clone_tail();

                self.calculate_new_head();
            }



        });

        return head_handle;
    }

    fn clone_tail(&self) -> GameState{
        return self.game_state_tail.read().unwrap().clone();
    }
    // Pointless_optimum: Things could probably be sped with references instead of cloning.

    fn calculate_new_head(&mut self, initial_tail: GameState) -> GameState{
        let mut head_to_be = self.game_state_tail.clone();
        let first_head_to_sim = initial_tail.get_simmed_frame_index() + 1;
        let frames_to_sim = first_head_to_sim..(first_head_to_sim + HEAD_AHEAD_FRAME_COUNT);
        { // Get all information needed from frame database.
            let all_frames = self.all_frames.read().unwrap();
            let mut players_last_input = all_frames.calculate_last_inputs();

        }

        for frame_index_to_simulate in first_head_to_sim..(first_head_to_sim + HEAD_AHEAD_FRAME_COUNT){
            let mut inputs_to_sim_with = HashMap::new();
            for (player_id,player_record) in self.all_frames.read().unwrap().player_inputs.iter(){
                let player_inputs = player_record.get_single_item(frame_index_to_simulate);
                match player_inputs{
                    Some(inputs) => {
                        inputs_to_sim_with.insert(*player_id, inputs.clone());
                    },
                    None => {
                        inputs_to_sim_with.insert(*player_id, players_last_input.get(player_id).unwrap().clone());
                    }
                }
            }
            {
                let frame_data = self.all_frames.read().unwrap();
                let bonus_infos = frame_data.bonus_events.get_single_item(frame_index_to_simulate);
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

        }
        {
            *self.game_state_head.lock().unwrap() = head_to_be; // Update mutex lock.
        }
    }

    //    pub fn load_frames(&mut self, storage: LogicDataStorage){
//        self.all_frames = storage;
//    }
    pub fn run_logic_loop(mut self, mut game_messages_channel: Receiver<LogicInwardsMessage>){
        let mut generator = self.known_frame_info.start_frame_stream_from_any(self.game_state_tail.get_simmed_frame_index());
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
                        self.outwards_messages.send( LogicOutwardsMessage::DataNeeded(request) ).unwrap();
                        std::thread::sleep(Duration::from_millis(100)); // Wait to save CPU cycles. modival: Can optimise recovery time by increasing check rate, but resend rate shouldn't be too high cos can't have too many messages.
                    }
                }
            }
            if self.head_is_ahead { // Don't bother on the server.
                self.calculate_new_head(tail_frame_to_sim);
            }else{
                self.set_head_to_tail(); // TODO3: Not sure why this is needed. Investigate.
            }
        }
    }
}

