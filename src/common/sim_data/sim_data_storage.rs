
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::common::gameplay::game::game_state::*;
use crate::common::sim_data::input_state::*;
use crate::common::sim_data::superstore_seg::*;

use crate::common::types::*;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::sync::mpsc::{Sender, channel};
use std::thread;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuerySimData {
    pub frame_offset: FrameIndex,
    pub player_id: PlayerID
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OwnedSimData {
    pub player_id: PlayerID,
    pub sim_data: SuperstoreData<InputState>
}


#[derive(Clone)]
pub struct SimDataStorageEx {
    player_inputs: ArcRw<HashMap<PlayerID, SuperstoreEx<InputState>>>,
    tail_simed_index: ArcRw<i32>
}
impl SimDataStorageEx{
    pub fn new(existing_players: Vec<PlayerID>, first_frame_to_store: FrameIndex) -> SimDataStorageEx{

        let mut storage = SimDataStorageEx {
            player_inputs: Default::default(),
            tail_simed_index: Arc::new(RwLock::new(-1))
        };
        for existing_player in existing_players{
            storage.init_new_player(existing_player, first_frame_to_store);
        }
        println!("Done pre-init existing players.");
        storage
    }
    pub fn set_tail_frame(&self, tail_frame: i32){
        *self.tail_simed_index.write().unwrap() = tail_frame;
    }
    pub fn get_player_list(&self, filter_must_have_inputs_on: FrameIndex) -> Vec<PlayerID>{
        let mut players = vec![];

        for (player_id, storage) in self.player_inputs.read().unwrap().iter(){
            if filter_must_have_inputs_on >= storage.get_first_frame_index(){
                players.push(*player_id);
            }
        }
        return players;
    }
    fn read_data(&self) -> RwLockReadGuard<HashMap<PlayerID, SuperstoreEx<InputState>>>{
        return self.player_inputs.read().unwrap();
    }
    pub fn init_new_player(&self, player_id: PlayerID, frame_offset: FrameIndex){
        println!("Creating new superstore for new player {}", player_id);
        let mut players_writable = self.player_inputs.write().unwrap();

        let new_superstore = SuperstoreEx::start(frame_offset);
        players_writable.insert(player_id, new_superstore);
    }

    pub fn write_data(&self, player_id: PlayerID, data: SuperstoreData<InputState>){

        let players = self.read_data();

        let players_containing_target_player = if players.contains_key(&player_id){
            players
        }else{
            // On new player, we do want to read, then write, then read again. This doesn't happen often.
            std::mem::drop(players); // So can write to.
            // Existing players should have been initialized in the 'ExistingPlayers' list in the welcome message - therefor all new players should have the new player flag.
            assert!(data.data.get(0).unwrap().new_player, "New data for unknown player {} which didn't have 'newplayer' flag set on first input. Drastic packet misordering might cause this, so we can remove this assert and just ignore instead.", player_id);
            self.init_new_player(player_id, data.frame_offset);
            self.read_data()
        };
        players_containing_target_player.get(&player_id).unwrap().write_requests_sink.lock().unwrap().send(data).unwrap();
    }
    pub fn write_data_single(&self, player_id: PlayerID, state: InputState, frame_index: FrameIndex){
        let data = SuperstoreData{
            data: vec![state],
            frame_offset: frame_index
        };
        self.write_data(player_id, data);
    }
    pub fn write_owned_data(&self, response: OwnedSimData){
        self.write_data(response.player_id, response.sim_data);
    }

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
    pub fn clone_info_for_tail(&self, frame_index: FrameIndex) -> Result<InfoForSim, Vec<QuerySimData>>{
        let mut player_inputs: HashMap<PlayerID, InputState> = Default::default();
        let mut problems = vec![];
        for (player_id, superstore) in self.read_data().iter(){
            if frame_index >= superstore.get_first_frame_index(){ // If we're not talking about before the player joined.
                match superstore.get_clone(frame_index){
                    Some(state) => {
                        player_inputs.insert(*player_id, state);
                    }
                    None => {
                        problems.push(QuerySimData {
                            frame_offset: frame_index,
                            player_id: *player_id
                        });
                    }
                }
            }

        }
        if problems.is_empty(){
            return Ok(InfoForSim{
                inputs_map: player_inputs
            });
        }else{
            return Err(problems);
        }

    }
    pub fn fulfill_query(&self, query: &QuerySimData) -> OwnedSimData {
        let players = self.read_data();
        let superstore = players.get(&query.player_id).expect("Can't find data for player.");

        let mut query_response = vec![];

        let slice_first_frame = query.frame_offset.max(superstore.get_first_frame_index());
        for target_index in slice_first_frame..(slice_first_frame + 20){ // modival Amount of data returned from an 'I'm missing data!' request, and how many of your last inputs get sent.
            let input_maybe = superstore.get_clone(target_index); // pointless_optimum: Shouldn't need to clone, but this'll likely be a painful fix.
            match input_maybe{
                Some(input) => {
                    query_response.push(input);
                }
                None => {
                    break;
                }
            }
        }

        OwnedSimData {
            player_id: query.player_id,
            sim_data: SuperstoreData { data: query_response, frame_offset: slice_first_frame }
        }
    }
}

