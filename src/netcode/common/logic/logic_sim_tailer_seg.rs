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
    fn get_server_events(&self, data_store: &SimDataStorage) -> Result<ServerEvents, Vec<SimDataQuery>>{
        let frame_to_sim = self.game_state.get_simmed_frame_index() + 1;
        let server_events = match data_store.get_server_events(frame_to_sim) {
            Some(events) => {
                return Ok(
                    events.clone());
            }
            None => {
                return Err(vec![SimDataQuery{
                    query_type : SimDataOwner::Server,
                    frame_offset : frame_to_sim,
                }]);
            }
        };
    }
    fn get_player_inputs(&self, data_store: &SimDataStorage) -> Result<HashMap<PlayerID, InputState>, Vec<SimDataQuery>>{
        let frame_to_sim = self.game_state.get_simmed_frame_index() + 1;

        let mut player_inputs: HashMap<PlayerID, InputState> = Default::default();
        let mut problems = vec![];

        let connected_players = self.game_state.get_connected_players();
        for connected_player in connected_players {
            if let Some(input_state) = data_store.get_input(frame_to_sim, connected_player){
                player_inputs.insert(connected_player, input_state.clone());
            }else{
                problems.push(SimDataQuery {
                    query_type: SimDataOwner::Player(connected_player),
                    frame_offset: frame_to_sim,
                });
            }
        }
        if !problems.is_empty(){
            return Err(problems);
        }else{
            return Ok(player_inputs);
        }
    }
    fn simulate_tick(&mut self, data_store: &SimDataStorage) -> Option<Vec<SimDataQuery>>{
        let server_events = self.get_server_events(data_store).ok()?;
        self.game_state.update_connected_players(&server_events);
        let player_inputs = self.get_player_inputs(data_store).ok()?;

        let sim_data = InfoForSim{
            inputs_map: player_inputs,
            server_events
        };
        self.game_state.simulate_tick(sim_data, FRAME_DURATION_MILLIS);
        self.update_hash();
        return None;
    }

    pub fn catchup_simulation(&mut self, data_store: &SimDataStorage, sim_frame_up_to_and_including: FrameIndex) -> Option<Vec<SimDataQuery>>{
        const MAX_FRAMES_CATCHUP : usize = 3; // modival
        let first_frame_to_sim = self.game_state.get_simmed_frame_index() + 1;
        let last_frame_to_sim = sim_frame_up_to_and_including.min(first_frame_to_sim + MAX_FRAMES_CATCHUP);
        for frame_to_sim in first_frame_to_sim..(last_frame_to_sim + 1){
            self.simulate_tick(data_store)?;
        }
        return None;
    }
    fn update_hash(&mut self){
        self.hashes.insert(self.game_state.get_simmed_frame_index(), self.game_state.get_hash());
    }
    pub fn check_hash(&mut self, framed_hash: FramedHash){
        match self.hashes.get(&framed_hash.frame){
            None => {

            }
            Some(existing_hash) => {
                // dcwct TODO1. Renemaed assert!(*existing_hash == framed_hash.hash, format!("Out of sync! Frame index {}", framed_hash.frame));
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FramedHash{
    pub frame: FrameIndex,
    pub hash: HashType,
}
