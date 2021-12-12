use crate::netcode::common::sim_data::confirmed_data::{ConfirmedData, SimDataQuery, SimDataPackage};
use crate::netcode::common::sim_data::superstore_seg::Superstore;
use crate::netcode::{InputState, InfoForSim};
use crate::netcode::common::sim_data::net_game_state::NetGameState;
use crate::pub_types::{FrameIndex, PlayerID};
use std::collections::HashMap;

pub struct ClientDataStore{
    pub confirmed_data: ConfirmedData,
    pub predicted_local: Superstore<InputState>,
}

impl ClientDataStore{
    fn get_head_sim_data_single(&self, frame: FrameIndex) -> InfoForSim {
        let mut player_inputs: HashMap<PlayerID, InputState> = Default::default();
        for (player_id, superstore) in self.confirmed_data.get_player_list() {
            if frame_index >= superstore.get_first_frame_index() { // If we're not talking about before the player joined.
                // Get or last or default.
                let state = superstore.get_clone(frame_index).or_else(|| { superstore.get_last_clone() }).unwrap_or_default();

                player_inputs.insert(*player_id, state);
            }
        }
        return InfoForSim {
            inputs_map: player_inputs,
            server_events: vec![]
        }
    }
    pub fn get_head_sim_data(&self, frame_from: FrameIndex, frame_to : FrameIndex) -> Vec<InfoForSim> {
        let mut player_inputs: HashMap<PlayerID, InputState> = Default::default();
        for (player_id, superstore) in self.read_data().iter(){
            if frame_index >= superstore.get_first_frame_index() { // If we're not talking about before the player joined.
                // Get or last or default.
                let state = superstore.get_clone(frame_index).or_else(||{superstore.get_last_clone()}).unwrap_or_default();

                player_inputs.insert(*player_id, state);
            }
        }
        return InfoForSim{
            inputs_map: player_inputs
        }
    }
    pub fn fulfill_query(&self, query: &SimDataQuery, item_count: i32) -> SimDataPackage {
        todo!()
    }
    pub fn new() -> Self{
        Self{
            confirmed_data: ConfirmedData::new(),
            predicted_local: Superstore::new(false),
        }
    }

}