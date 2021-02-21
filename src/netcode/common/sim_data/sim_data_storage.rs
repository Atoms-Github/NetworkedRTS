use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::netcode::*;
use crate::netcode::common::sim_data::input_state::*;
use crate::netcode::common::sim_data::superstore_seg::*;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;

use std::sync::{Arc, RwLock, RwLockReadGuard};
use crossbeam_channel::*;
use std::thread;


#[derive(Serialize, Deserialize, Clone, Debug, Eq)]
pub enum ServerEvent{
    JoinPlayer(PlayerID),
    DisconnectPlayer(PlayerID)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SimDataOwner {
    Server,
    Player(PlayerID),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimDataQuery {
    pub query_type: SimDataOwner,
    pub frame_offset: FrameIndex,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SimDataPackage{
    ServerEvents(SuperstoreData<Vec<ServerEvent>>),
    PlayerInputs(SuperstoreData<InputState>, PlayerID)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimDataStorage {
    player_inputs: HashMap<PlayerID, Superstore<InputState>>,
    server_events: Superstore<Vec<ServerEvent>>,
    first_frame: FrameIndex,
}
impl SimDataStorage {
    pub fn new(first_frame_to_store: FrameIndex) -> Self {
        let mut storage = Self {
            player_inputs: Default::default(),
            server_events: Superstore::new(first_frame_to_store),
            first_frame: first_frame_to_store
        };
        storage
    }
    fn get_player_superstore(&mut self, player_id: PlayerID) -> &Superstore<InputState>{
        if !self.player_inputs.contains_key(&player_id){
            self.player_inputs.insert(player_id, Superstore::new(self.first_frame));
        }
        return self.player_inputs.get(&player_id).unwrap();
    }
    pub fn write_data(&mut self, data: SimDataPackage){
        match data{
            SimDataPackage::PlayerInputs(data, player_id) => {
                self.get_player_superstore(player_id).write_data(data);
            }
            SimDataPackage::ServerEvents(data) => {
                self.server_events.write_data(data);
            }
        }
    }
    pub fn write_input_data_single(&self, player_id: PlayerID, state: InputState, frame_index: FrameIndex){
        let package = SimDataPackage::PlayerInputs(SuperstoreData{
            data: vec![state],
            frame_offset: frame_index
        }, player_id);
    }
    pub fn fulfill_query(&self, query: &SimDataQuery) -> SimDataPackage {
        match query.query_type{
            SimDataOwner::Server => {
                SimDataPackage::ServerEvents(SuperstoreData{
                    data: self.server_events.clone_block(query.frame_offset, 20),
                    frame_offset: query.frame_offset
                })
            }
            SimDataOwner::Player(player_id) => {
                let superstore = self.player_inputs.get(&query.player_id).expect("DataStore was queried for a player it didn't know existed.");
                SimDataPackage::PlayerInputs(SuperstoreData{
                    data: superstore.clone_block(query.frame_offset, 20),
                    frame_offset: query.frame_offset
                }, player_id)
            }
        }

    }
}

/*


pub fn clone_info_for_head(&self, frame_index: FrameIndex) -> InfoForSim{
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
    pub fn clone_info_for_tail(&self, frame_index: FrameIndex, who_we_wait_for: Vec<PlayerID>) -> Result<InfoForSim, Vec<SimDataQuery>>{
        // We need to make sure that everyone who we need to wait for is in, then we return with a list of everyone.
        // This means newly joined players will be returned, so can be waited for next frame.

        // This is how its designed - we're chucking a list of inputs into a list from all sources and threads. We don't have a notification for new join.
        // We need to check every check to see if someone new joined.

        let data = self.read_data();

        let mut player_inputs: HashMap<PlayerID, InputState> = Default::default();
        let mut problems = vec![];
        // Make sure we've got the inputs we need.
        for waiting_id in who_we_wait_for{
            let superstore = data.get(&waiting_id).expect("Asked to wait on a player who didn't exist.");
            match superstore.get_clone(frame_index){
                Some(state) => {
                    // Nothing. Gets added later.
                }
                None => {
                    problems.push(SimDataQuery {
                        frame_offset: frame_index,
                        player_id: waiting_id
                    });
                }
            }
        }

        if !problems.is_empty(){
            return Err(problems);

        }

        // Gather all the inputs we have.
        for (player_id, superstore) in data.iter(){
            match superstore.get_clone(frame_index){
                Some(state) => {
                    player_inputs.insert(*player_id, state);
                }
                None => {
                    // Don't care if non-waiting required player doesn't have inputs.
                }
            }
        }
        return Ok(InfoForSim{
            inputs_map: player_inputs
        });
    }

 */