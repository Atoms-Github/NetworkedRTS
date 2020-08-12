
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
    pub fn new() -> SimDataStorageEx{
        SimDataStorageEx{
            player_inputs: Default::default(),
            tail_simed_index: Arc::new(RwLock::new(-1))
        }
    }
    pub fn set_tail_frame(&self, tail_frame: i32){
        *self.tail_simed_index.write().unwrap() = tail_frame;
    }
    fn read_data(&self) -> RwLockReadGuard<HashMap<PlayerID, SuperstoreEx<InputState>>>{
        return self.player_inputs.read().unwrap();
    }
    fn init_new_player(&self, player_id: PlayerID, frame_offset: FrameIndex){
        println!("Creating new superstore for new player {}", player_id);
        let mut players_writable = self.player_inputs.write().unwrap();

        let new_superstore = SuperstoreEx::start(frame_offset, self.tail_simed_index.clone());
        players_writable.insert(player_id, new_superstore);
    }

    pub fn write_data(&self, player_id: PlayerID, data: SuperstoreData<InputState>){

        let players = self.read_data();

        let players_containing_target_player = if players.contains_key(&player_id){
            players
        }else{
            // On new player, we do want to read, then write, then read again. This doesn't happen often.
            std::mem::drop(players); // So can write to.
            assert!(data.data.get(0).unwrap().new_player, "New data for unknown player which didn't have 'newplayer' flag set on first input. Packet misordering might cause this, so we can remove this assert and just ignore instead.");
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


//    pub fn handle_inwards_msg(&mut self, msg: LogicInwardsMessage){
//        match msg{
//            LogicInwardsMessage::SyncerInputsUpdate(data) => {
//                let player_sync = self.player_inputs.entry(data.owning_player).or_insert_with(|| FramedVec::<InputState>::new(data.start_frame));
//                player_sync.insert_data_segment(data);
//            }
//        }
//    }
//    pub fn clone_info_for_sim(&self, frame_index: FrameIndex) -> SimDataQueryResults{
//        // Should return that there's no error when getting value before player inited but just blank
//        let mut missing_item_requests = vec![];
////        let (bonus_list, problem_bonus) = ;
//
//        let mut latest_inputs = HashMap::new();
//        for (player_id, data) in self.player_inputs.iter(){
//            if frame_index < data.frames_index_offset{
//                continue;
//            }
//            let (player_inputs, problem_inputs) =
//                data.get_or_last_query(frame_index, FramedVecRequestType::PlayerInputs(*player_id));
//
//            latest_inputs.insert(*player_id, player_inputs.unwrap_or_default());
//
//            if let Some(request) = problem_inputs {
//                missing_item_requests.push(request);
//            }
//        }
//        SimDataQueryResults{
//            missing_info: missing_item_requests,
//            sim_info: InfoForSim {
//                inputs_map: latest_inputs
//            }
//        }
//    }
//
//
//
//
//
//
//
////    pub fn calculate_last_inputs(&self) -> HashMap<PlayerID, InputState>{
////        let mut to_return = HashMap::new();
////
////        for (player_id,player_record) in self.player_inputs.iter(){
////            let last_input= player_record.data.last();
////            let usable_input;
////            match last_input{
////                Some(state) => {
////                    usable_input = state.clone();
////                }
////                None => {
////                    usable_input = InputState::new();
////                }
////
////            }
////            to_return.insert(*player_id, usable_input);
////        }
////
////        return to_return;
////    }
////    pub fn get_frames_segment(&self, segment_needed: &Sync) -> Option<LogicInwardsMessage> {
////        match segment_needed.type_needed{
////            LogicInfoRequestType::PlayerInputs(player_id) => {
////                // Eventually..., this whole thing can probably be sped up by not cloning anywhere. Just using fancy lifetimed references.
////                let player_record = self.frames_map.get(&player_id)?; // Wayyyy, using question marks like a boss. :)
////                let relative_start_frame = segment_needed.start_frame - player_record.start_frame;
////
////
////                let mut input_states_found = vec![];
////                for relative_index in relative_start_frame..(relative_start_frame + segment_needed.number_of_frames /*No need for +1 */){
////                    let sync = player_record.sync.get(relative_index);
////                    if sync.is_some(){
////                        let input_segment = PlayerInputSegmentType::WholeState(sync.unwrap().clone());
////                        input_states_found.push(input_segment);
////                    }
////
////                }
////
////                return Some(LogicInwardsMessage::InputsUpdate(LogicInputsResponse{
////                    player_id,
////                    start_frame_index: segment_needed.start_frame,
////                    input_states: input_states_found
////                }));
////            }
////            LogicInfoRequestType::BonusEvents => {
////                // This should never be called on the client.
////                let mut events = vec![];
////                for abs_index in segment_needed.start_frame..(segment_needed.start_frame + segment_needed.number_of_frames){
////                    let relative_index = abs_index - self.bonus_start_frame;
////                    let events_list = self.bonus_events.get(abs_index);
////                    if events_list.is_some(){
////                        events.push(events_list.unwrap().clone());
////                    }else{
////                        break; // Reached end of list.
////                    }
////                }
////                let msg = LogicInwardsMessage::BonusMsgsUpdate(BonusMsgsResponse{
////                    start_frame_index: segment_needed.start_frame,
////                    event_lists: events
////                });
////                return Some(msg);
////            }
////        }
////    }
}

