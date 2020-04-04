use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, TryRecvError, Sender};
use serde::{Deserialize, Serialize};

use crate::game::timekeeping::*;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::game_message_types::{NewPlayerInfo};
use crate::network::networking_structs::*;
use crate::network::game_message_types::*;
use std::panic;
use std::collections::HashMap;
use std::thread::Thread;
use std::time::Duration;
use crate::game::synced_data_stream::*;
use crate::players::inputs::*;
use crate::game::logic::logic_segment::*;
use crate::game::bonus_msgs_segment::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogicDataStorage {
    pub player_inputs: HashMap<PlayerID, SyncerStore<InputState>>,
    pub bonus_events: SyncerStore<Vec<BonusEvent>>,
}
impl LogicDataStorage{
    pub fn handle_inwards_msg(&mut self, msg: LogicInwardsMessage){
        match msg{
            LogicInwardsMessage::SyncerInputsUpdate(data) => {
                let player_sync = self.player_inputs.get_mut(&data.owning_player).unwrap();
                player_sync.insert_data_segment(data);
            }
            LogicInwardsMessage::SyncerBonusUpdate(data) => {
                self.bonus_events.insert_data_segment(data);
            }
        }
    }
    pub fn new(frame_offset: FrameIndex) -> LogicDataStorage{
        LogicDataStorage{
            player_inputs: Default::default(),
            bonus_events: SyncerStore::gen_bonus_store(frame_offset),
        }
    }
    pub fn add_player(&mut self, new_player_info: &NewPlayerInfo){
        self.player_inputs.insert(new_player_info.player_id, SyncerStore::gen_inputs_store(new_player_info.frame_added));
    }
    pub fn get_simable_info(){

    }
    pub fn calculate_last_inputs(&self) -> HashMap<PlayerID, InputState>{
        let mut to_return = HashMap::new();

        for (player_id,player_record) in self.player_inputs.iter(){
            let last_input= player_record.data.last();
            let usable_input;
            match last_input{
                Some(state) => {
                    usable_input = state.clone();
                }
                None => {
                    usable_input = InputState::new();
                }

            }
            to_return.insert(*player_id, usable_input);
        }

        return to_return;
    }
//    pub fn get_frames_segment(&self, segment_needed: &Sync) -> Option<LogicInwardsMessage> {
//        match segment_needed.type_needed{
//            LogicInfoRequestType::PlayerInputs(player_id) => {
//                // Eventually..., this whole thing can probably be sped up by not cloning anywhere. Just using fancy lifetimed references.
//                let player_record = self.frames_map.get(&player_id)?; // Wayyyy, using question marks like a boss. :)
//                let relative_start_frame = segment_needed.start_frame - player_record.start_frame;
//
//
//                let mut input_states_found = vec![];
//                for relative_index in relative_start_frame..(relative_start_frame + segment_needed.number_of_frames /*No need for +1 */){
//                    let inputs = player_record.inputs.get(relative_index);
//                    if inputs.is_some(){
//                        let input_segment = PlayerInputSegmentType::WholeState(inputs.unwrap().clone());
//                        input_states_found.push(input_segment);
//                    }
//
//                }
//
//                return Some(LogicInwardsMessage::InputsUpdate(LogicInputsResponse{
//                    player_id,
//                    start_frame_index: segment_needed.start_frame,
//                    input_states: input_states_found
//                }));
//            }
//            LogicInfoRequestType::BonusEvents => {
//                // This should never be called on the client.
//                let mut events = vec![];
//                for abs_index in segment_needed.start_frame..(segment_needed.start_frame + segment_needed.number_of_frames){
//                    let relative_index = abs_index - self.bonus_start_frame;
//                    let events_list = self.bonus_events.get(abs_index);
//                    if events_list.is_some(){
//                        events.push(events_list.unwrap().clone());
//                    }else{
//                        break; // Reached end of list.
//                    }
//                }
//                let msg = LogicInwardsMessage::BonusMsgsUpdate(BonusMsgsResponse{
//                    start_frame_index: segment_needed.start_frame,
//                    event_lists: events
//                });
//                return Some(msg);
//            }
//        }
//    }
}

