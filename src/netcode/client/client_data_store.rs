use crate::pub_types::{FrameIndex, PlayerID};
use std::collections::HashMap;
use crate::netcode::common::input_state::InputState;
use crate::netcode::common::confirmed_data::{ConfirmedData, SimDataQuery, SimDataPackage};
use crate::netcode::common::superstore_seg::Superstore;
use crate::netcode::InfoForSim;

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
            }else if let Some(input_state) = self.confirmed_data.get_last_input(player){
                player_inputs.insert(player, input_state.clone());
            }
        }
        // Overwrite my own:
        if let Some(input_state) = self.predicted_local.get(frame){
            player_inputs.insert(self.my_player_id, input_state.clone());
        }else if let Some(last_frame) = self.predicted_local.last_frame{
            if let Some(input_state) = self.predicted_local.get(last_frame){
                player_inputs.insert(self.my_player_id, input_state.clone());
            }
        }

        return InfoForSim {
            inputs_map: player_inputs,
            server_events: self.confirmed_data.get_server_events_or_empty(frame)
        }
    }
    pub fn get_head_sim_data(&self, frame_from: FrameIndex, frame_to : FrameIndex) -> Vec<InfoForSim> {
        return (frame_from..(frame_to + 1)).map(|frame|{self.get_head_sim_data_single(frame)}).collect();
    }
    pub fn fulfill_query(&self, query: &SimDataQuery, item_count: usize) -> SimDataPackage {
        let mut results = self.confirmed_data.fulfill_query(query, item_count);
        // Now modify results with self, if required.
        if let SimDataPackage::PlayerInputs(data, player) = &mut results{
            if *player == self.my_player_id{
                let predicted_block = self.predicted_local.clone_block(query.frame_offset, item_count);
                data.data = predicted_block;
            }
        }

        return results;
    }
    pub fn new(my_player_id: PlayerID) -> Self{
        let mut confirmed_data = ConfirmedData::new();
        confirmed_data.register_player(my_player_id);
        Self{
            my_player_id,
            confirmed_data,
            predicted_local: Superstore::new(false),
        }
    }

}