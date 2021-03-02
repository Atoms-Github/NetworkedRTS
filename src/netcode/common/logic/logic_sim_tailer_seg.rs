use std::thread;
use std::sync::{Arc, RwLock};
use crossbeam_channel::*;

use serde::{Deserialize, Serialize};

use crate::netcode::*;
use crate::netcode::common::sim_data::input_state::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use crate::netcode::common::sim_data::superstore_seg::*;
use crate::netcode::common::sim_data::sim_data_storage::*;
use std::time::{SystemTime, Duration};

use crate::netcode::common::logic::hash_seg::*;
use std::hash::Hash;
use crate::netcode::common::sim_data::net_game_state::{NetPlayerProperty, NetGameState};


pub struct LogicSimTailer {
    pub game_state: NetGameState,
    pub known_frame: KnownFrameInfo,
}
impl LogicSimTailer{
    pub fn new(game_state: NetGameState, known_frame: KnownFrameInfo) -> Self{
        Self{
            game_state,
            known_frame
        }
    }
    // breaking: Need to be able to sim two in one call.
    fn get_info_for_sim(&mut self, data_store: &SimDataStorage) -> Result<InfoForSim, Vec<SimDataQuery>>{
        let frame_to_sim = self.game_state.get_simmed_frame_index() + 1;

        let mut player_inputs: HashMap<PlayerID, InputState> = Default::default();
        let mut problems = vec![];

        for (player_id, player_property) in self.game_state.players{
            if let Some(input_state) = data_store.get_input(frame_to_sim, player_id){
                player_inputs.insert(*player_id, state);
            }else{
                problems.push(SimDataQuery {
                    query_type: SimDataOwner::Player(),
                    frame_offset: frame_index,
                    player_id: waiting_id
                });
            }
        }
        // breaking get server events too.

        if !problems.is_empty(){
            return Err(problems);

        }
        return Ok(InfoForSim{
            inputs_map: player_inputs
        });
    }
    pub fn catchup_simulation(&mut self, data_store: &SimDataStorage, sim_frame_up_to_and_including: FrameIndex) -> Result<(), Vec<SimDataQuery>>{
        let next_frame = self.game_state.get_simmed_frame_index() + 1;
        for _ in next_frame..(sim_frame_up_to_and_including + 1).min(next_frame + 4 /*Modival. Max catch up 3 frames.*/){
            let sim_info_result = self.get_info_for_sim(data_store);
            self.game_state.simulate_tick()
        }
        return ();
        // breaking catchup a bit using self.known_frame. Also implement limit of 3 sims.
    }
}