use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::*;
use crate::netcode_types::*;
use crate::pub_types::*;

use std::sync::{Arc, RwLock, RwLockReadGuard};
use crossbeam_channel::*;
use std::thread;
use nalgebra::{DimAdd};
use crate::common::superstore_seg::{SuperstoreData, Superstore};
use crate::common::input_state::InputState;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServerEvent{
    JoinPlayer(PlayerID, String, Shade),
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
impl SimDataPackage{
    pub fn get_size(&self) -> usize{
        match self{
            SimDataPackage::ServerEvents(events) => {events.data.len()}
            SimDataPackage::PlayerInputs(inputs, _) => {inputs.data.len()}
        }
    }
    pub fn get_frame(&self) -> FrameIndex{
        match self{
            SimDataPackage::ServerEvents(events) => {events.frame_offset}
            SimDataPackage::PlayerInputs(inputs, _) => {inputs.frame_offset}
        }
    }
    pub fn new_single_server(frame: FrameIndex, events: ServerEvents) -> Self{
        Self::ServerEvents(SuperstoreData{
            data: vec![events],
            frame_offset: frame
        })
    }
    pub fn new_single_player(frame: FrameIndex, player: PlayerID, input: InputState) -> Self{
        Self::PlayerInputs(SuperstoreData{
            data: vec![input],
            frame_offset: frame
        }, player)
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfirmedData {
    pub player_inputs: HashMap<PlayerID, Superstore<InputState>>,
    pub server_events: Superstore<ServerEvents>,
}
impl ConfirmedData {
    pub fn new() -> Self {
        let mut storage = Self {
            player_inputs: Default::default(),
            server_events: Superstore::new(true),
        };
        storage
    }
    
    pub fn get_input(&self, frame_index: FrameIndex, player_id: PlayerID) -> Option<&InputState>{
        self.player_inputs.get(&player_id)?.get(frame_index)
    }
    pub fn get_last_input(&self, player_id: PlayerID) -> Option<&InputState>{
        self.get_input(self.get_last_input_frame(player_id)?, player_id)
    }
    pub fn get_last_input_frame(&self, player_id: PlayerID) -> Option<FrameIndex>{
        self.player_inputs.get(&player_id)?.last_frame
    }
    pub fn get_server_events(&self, frame_index: FrameIndex) -> Option<&ServerEvents>{
        self.server_events.get(frame_index)
    }
    pub fn get_server_events_or_empty(&self, frame_index: FrameIndex) -> ServerEvents{
        self.get_server_events(frame_index).cloned().unwrap_or_else(||{vec![]})
    }
    pub fn get_player_list(&self) -> Vec<PlayerID>{
        return self.player_inputs.keys().cloned().collect();
    }
    pub fn register_player(&mut self, player: PlayerID){
        self.player_inputs.insert(player, Superstore::new(true));
    }
    pub fn write_data(&mut self, data: SimDataPackage){
        match data{
            SimDataPackage::PlayerInputs(data, player_id) => {
                self.player_inputs.entry(player_id)
                    .or_insert_with(||{Superstore::new(true)}).write_data(data);
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
        return self.server_events.last_frame.unwrap_or_else(||{0}) + 1;
    }
    pub fn fulfill_query(&self, query: &SimDataQuery, number_of_items: usize) -> SimDataPackage {
        match query.query_type{
            SimDataOwner::Server => {
                SimDataPackage::ServerEvents(SuperstoreData{
                    data: self.server_events.clone_block(query.frame_offset, number_of_items),
                    frame_offset: query.frame_offset
                })
            }
            SimDataOwner::Player(player_id) => {
                let store = self.player_inputs.get(&player_id);
                assert!(store.is_some(), "DataStore was queried for a player it didn't know existed. {}", player_id);
                SimDataPackage::PlayerInputs(SuperstoreData{
                    data: store.unwrap().clone_block(query.frame_offset, number_of_items),
                    frame_offset: query.frame_offset
                }, player_id)
            }
        }
    }

    pub fn server_connect_player(&mut self, player_id: PlayerID, name: String, color: Shade) -> Vec<SimDataPackage>{
        let join = SimDataPackage::ServerEvents(SuperstoreData{
            data: vec![vec![ServerEvent::JoinPlayer(player_id, name, color)]],
            frame_offset: self.get_next_empty_server_events_frame()
        });
        return vec![join];
    }
    pub fn server_disconnect_player(&mut self, player_id: PlayerID, tail_last_simmed: FrameIndex) -> Vec<SimDataPackage>{
        let kick_event = SimDataPackage::ServerEvents(SuperstoreData{
            data: vec![vec![ServerEvent::DisconnectPlayer(player_id)]],
            frame_offset: self.get_next_empty_server_events_frame()
        });
        let mut from_frame = 0;
        if let Some(superstore) = self.player_inputs.get_mut(&player_id){
            if let Some(frame) = superstore.last_frame{
                from_frame = frame + 1;
            }
        }
        from_frame = from_frame.max(tail_last_simmed + 1);

        let mut dummy_inputs = vec![];
        // Now to fill up that player's inputs with garbo.
        for frame_index in from_frame..(kick_event.get_frame()){
            if let Some(input) = self.get_last_input(player_id){
                dummy_inputs.push(input.clone());
            }
        }
        let empty_messages = SimDataPackage::PlayerInputs(SuperstoreData{
            data: dummy_inputs,
            frame_offset: from_frame
        }, player_id);
        return vec![kick_event, empty_messages];
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