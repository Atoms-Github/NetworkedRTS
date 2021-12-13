use crate::netcode::common::sim_data::confirmed_data::{ConfirmedData, SimDataQuery, SimDataPackage};
use crate::netcode::common::sim_data::superstore_seg::Superstore;
use crate::netcode::{InputState, InfoForSim};
use crate::pub_types::{FrameIndex, PlayerID};
use std::collections::HashMap;

pub struct ClientDataStore{
    pub my_player_id: PlayerID,
    pub confirmed_data: ConfirmedData,
    pub predicted_local: Superstore<InputState>,
}

impl ClientDataStore{
    fn get_head_sim_data_single(&self, frame: FrameIndex) -> InfoForSim {
        let mut player_inputs: HashMap<PlayerID, InputState> = Default::default();
        // We should just copy what we can, and ignore what we can't. Don't bother with blanks,
        // We're going to replace keyless with blanks later anyway.
        for player in self.confirmed_data.get_player_list() {
            if let Some(input_state) = self.confirmed_data.get_input(frame, player){
                player_inputs.insert(player, input_state.clone());
            }
        }
        // Overwrite my own:
        if let Some(input_state) = self.predicted_local.get(frame){
            player_inputs.insert(self.my_player_id, input_state.clone());
        }

        return InfoForSim {
            inputs_map: player_inputs,
            server_events: self.confirmed_data.get_server_events_or_empty(frame)
        }
    }
    pub fn get_head_sim_data(&self, frame_from: FrameIndex, frame_to : FrameIndex) -> Vec<InfoForSim> {
        return (frame_from..(frame_to + 1)).map(|frame|{self.get_head_sim_data_single(frame)}).collect();
    }
    pub fn fulfill_query(&self, query: &SimDataQuery, item_count: i32) -> SimDataPackage {
        todo!()
    }
    pub fn new(my_player_id: PlayerID) -> Self{
        Self{
            my_player_id,
            confirmed_data: ConfirmedData::new(),
            predicted_local: Superstore::new(false),
        }
    }

}