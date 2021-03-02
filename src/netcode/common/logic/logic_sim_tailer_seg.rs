use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, SystemTime};

use crossbeam_channel::*;
use serde::{Deserialize, Serialize};
use serde::__private::de::missing_field;

use crate::netcode::*;
use crate::netcode::common::sim_data::input_state::*;
use crate::netcode::common::sim_data::net_game_state::{NetGameState, NetPlayerProperty};
use crate::netcode::common::sim_data::sim_data_storage::*;
use crate::netcode::common::sim_data::superstore_seg::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;

pub struct LogicSimTailer {
    pub game_state: NetGameState,
    pub known_frame: KnownFrameInfo,
    pub hashes: HashMap<FrameIndex, HashType>, // pointless_optimum Could use vec, but easier to use hashmap.
}
impl LogicSimTailer{
    pub fn new(game_state: NetGameState, known_frame: KnownFrameInfo) -> Self{
        Self{
            game_state,
            known_frame,
            hashes: Default::default()
        }
    }
    fn get_info_for_sim(&mut self, data_store: &SimDataStorage, frame_to_sim: FrameIndex) -> Result<InfoForSim, Vec<SimDataQuery>>{
        let mut player_inputs: HashMap<PlayerID, InputState> = Default::default();
        let mut problems = vec![];

        for (player_id, player_property) in &self.game_state.players{
            if let Some(input_state) = data_store.get_input(frame_to_sim, *player_id){
                player_inputs.insert(*player_id, input_state.clone());
            }else{
                problems.push(SimDataQuery {
                    query_type: SimDataOwner::Player(*player_id),
                    frame_offset: frame_to_sim,
                });
            }
        }

        let server_events = data_store.get_server_events(frame_to_sim);
        if server_events.is_none(){
            problems.push(SimDataQuery{
                query_type : SimDataOwner::Server,
                frame_offset : frame_to_sim,
            })
        }

        if !problems.is_empty(){
            return Err(problems);
        }
        return Ok(InfoForSim{
            inputs_map: player_inputs,
            server_events: server_events.unwrap().clone()
        });
    }
    fn update_hash(&mut self){
        self.hashes.insert(self.game_state.get_simmed_frame_index(), self.game_state.get_hash());
    }
    pub fn check_hash(&mut self, framed_hash: FramedHash){
        match self.hashes.get(&framed_hash.frame){
            None => {

            }
            Some(existing_hash) => {
                assert!(*existing_hash == framed_hash.hash, format!("Out of sync! Frame index {}", framed_hash.frame));
            }
        }
    }
    pub fn catchup_simulation(&mut self, data_store: &SimDataStorage, sim_frame_up_to_and_including: FrameIndex) -> Result<(), Vec<SimDataQuery>>{
        const MAX_FRAMES_CATCHUP : usize = 3; // modival
        let first_frame_to_sim = self.game_state.get_simmed_frame_index() + 1;
        let last_frame_to_sim = sim_frame_up_to_and_including.min(first_frame_to_sim + MAX_FRAMES_CATCHUP);
        for frame_to_sim in first_frame_to_sim..(last_frame_to_sim + 1){
            let sim_info_result = self.get_info_for_sim(data_store, frame_to_sim);

            match sim_info_result{
                Ok(sim_info) => {
                    self.game_state.simulate_tick(sim_info, FRAME_DURATION_MILLIS);
                    self.update_hash();
                }
                Err(missing_info) => {
                    return Err(missing_info);
                }
            }
        }
        return Ok(());
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FramedHash{
    pub frame: FrameIndex,
    pub hash: HashType,
}
