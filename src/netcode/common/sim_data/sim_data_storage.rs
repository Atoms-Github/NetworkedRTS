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
use nalgebra::sup;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServerEvent{
    JoinPlayer(PlayerID),
    DisconnectPlayer(PlayerID),
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
    ServerEvents(SuperstoreData<ServerEvents>),
    PlayerInputs(SuperstoreData<InputState>, PlayerID)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimDataStorage {
    player_inputs: HashMap<PlayerID, Superstore<InputState>>,
    server_events: Superstore<ServerEvents>,
}
impl SimDataStorage {
    pub fn new(first_frame_to_store: FrameIndex) -> Self {
        let mut storage = Self {
            player_inputs: Default::default(),
            server_events: Superstore::new(first_frame_to_store),
        };
        storage
    }

    fn get_player_superstore(&mut self, player_id: PlayerID, first_frame_if_no_exist: FrameIndex) -> &Superstore<InputState>{
        if !self.player_inputs.contains_key(&player_id){
            self.player_inputs.insert(player_id, Superstore::new(first_frame_if_no_exist));
        }
        return self.player_inputs.get(&player_id).unwrap();
    }
    fn get_player_superstore_mut(&mut self, player_id: PlayerID, first_frame_if_no_exist: FrameIndex) -> &mut Superstore<InputState>{
        if !self.player_inputs.contains_key(&player_id){
            self.player_inputs.insert(player_id, Superstore::new(first_frame_if_no_exist));
        }
        return self.player_inputs.get_mut(&player_id).unwrap();
    }
    pub fn get_input(&self, frame_index: FrameIndex, player_id: PlayerID) -> Option<&InputState>{
        if let Some(superstore) = self.player_inputs.get(&player_id){
            return superstore.get(frame_index);
        }
        return None;
    }
    pub fn get_server_events(&self, frame_index: FrameIndex) -> Option<&ServerEvents>{
        return self.server_events.get(frame_index);
    }
    pub fn get_server_events_or_empty(&self, frame_index: FrameIndex) -> ServerEvents{
        return self.server_events.get(frame_index).cloned().unwrap_or_else(||{vec![]});
    }
    pub fn get_player_list(&self) -> Vec<PlayerID>{
        return self.player_inputs.keys().cloned().collect();
    }
    pub fn write_data(&mut self, data: SimDataPackage){
        match data{
            SimDataPackage::PlayerInputs(data, player_id) => {
                self.get_player_superstore_mut(player_id, data.frame_offset).write_data(data);
            }
            SimDataPackage::ServerEvents(data) => {
                self.server_events.write_data(data);
            }
        }
    }
    pub fn write_input_data_single(&mut self, player_id: PlayerID, state: InputState, frame_index: FrameIndex){
        let package = SimDataPackage::PlayerInputs(SuperstoreData{
            data: vec![state],
            frame_offset: frame_index
        }, player_id);
        self.write_data(package);
    }
    pub fn get_next_empty_server_events_frame(&self) -> FrameIndex{
        return self.server_events.get_next_empty_frame();
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
                let superstore = self.player_inputs.get(&player_id).expect("DataStore was queried for a player it didn't know existed.");
                SimDataPackage::PlayerInputs(SuperstoreData{
                    data: superstore.clone_block(query.frame_offset, 20),
                    frame_offset: query.frame_offset
                }, player_id)
            }
        }
    }
    pub fn schedule_server_event(&mut self, server_event: ServerEvent) -> FrameIndex{
        let event_frame = self.get_next_empty_server_events_frame();

        log::info!("Server scheduled a new server event on frame {}! {:?}", event_frame, server_event);

        let events_on_frame = vec![server_event];
        let data = SuperstoreData{
            data: vec![events_on_frame],
            frame_offset: event_frame
        };
        let package = SimDataPackage::ServerEvents(data);
        self.write_data(package);
        return event_frame;
    }
    pub fn server_boot_player(&mut self, player_id: PlayerID, tail_last_simmed: FrameIndex){
        let frame_booted = self.schedule_server_event(ServerEvent::DisconnectPlayer(player_id));
        let frame_to_fill_from= self.get_player_superstore(player_id, tail_last_simmed).get_next_empty_frame();

        // Now to fill up that player's inputs with garbo.
        for frame_index in frame_to_fill_from..(frame_booted + 1){
            self.write_input_data_single(player_id, InputState::new(), frame_index)
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