
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
pub struct SimDataStorageIn {
    write_sinks: HashMap<PlayerID, Sender<InputState>>
}

#[derive(Clone)]
pub struct SimDataStorageEx {
    player_inputs: ArcRw<HashMap<PlayerID, SuperstoreEx<InputState>>>,
    write_sink: Sender<OwnedSimData>
}
impl SimDataStorageIn{
    pub fn new() -> Self{
        SimDataStorageIn{
            write_sinks: Default::default()
        }
    }
    pub fn start(self) -> SimDataStorageEx{
        let (write_sink, write_rec) = channel();
        thread::spawn(move||{
            loop{
                let owned_msg = write_rec.recv().unwrap();

                self.write_sinks.
            }
        });
        SimDataStorageEx{
            player_inputs: Default::default(),
            write_sink
        }
    }
}
impl SimDataStorageEx{
    fn read_data(&self) -> RwLockReadGuard<HashMap<PlayerID, SuperstoreEx<InputState>>>{
        return self.player_inputs.read().unwrap();
    }

    pub fn write_data(&self, player_id: PlayerID, data: SuperstoreData<InputState>){
        let superstore = self.read_data().get(&player_id).expect("Can't find data for player.");
        superstore.write_requests_sink.send(data).unwrap();
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
        let player_inputs = Default::default();
        for (player_id, superstore) in self.read_data().iter(){
            // Get or last or default.
            let state = match superstore.get(frame_index).or_else(||{superstore.get_last()}){
                Some(state_ref) => {
                    state_ref.clone()
                }
                None => {
                    Default::default()
                }
            };
            player_inputs.insert(player_id, state);
        }
        return InfoForSim{
            inputs_map: player_inputs
        }
    }
    pub fn clone_info_for_tail(&self, frame_index: FrameIndex) -> Result<InfoForSim, Vec<QuerySimData>>{
        let player_inputs = Default::default();
        let mut problems = vec![];
        for (player_id, superstore) in self.read_data().iter(){
            match superstore.get(frame_index){
                Some(state) => {
                    player_inputs.insert(player_id, state.clone())
                }
                None => {
                    problems.push(QuerySimData {
                        frame_offset: frame_index,
                        player_id: *player_id
                    });
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
        let superstore = self.read_data().get(*query.player_id).expect("Can't find data for player.");

        let mut query_response = vec![];
        for i in 0..10{ // modival Amount of data returned from an 'I'm missing data!' request.
            query_response.push(superstore.get(query.frame_offset + i).unwrap().clone()); // pointless_optimum: Shouldn't need to clone, but this'll likely be a painful fix.
        }

        OwnedSimData {
            player_id: query.player_id,
            sim_data: SuperstoreData { data: vec![], frame_offset: query.frame_offset }
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
