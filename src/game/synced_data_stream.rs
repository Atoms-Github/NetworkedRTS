
use serde::{Deserialize, Serialize};

use crate::game::bonus_msgs_segment::*;
use crate::network::networking_structs::*;
use crate::players::inputs::*;
use crate::utils::util_functions::*;
use crate::network::game_message_types::NewPlayerInfo;


pub trait ExtractNewPlayers{
    fn extract_new_players(&self) -> Vec<NewPlayerInfo>;
}
impl ExtractNewPlayers for SyncerData<Vec<BonusEvent>>{
    fn extract_new_players(&self) -> Vec<NewPlayerInfo> {
        let mut new_players = vec![];
        for (package_relative_frame_index, bonus_list) in self.data.iter().enumerate(){
            for bonus_event in bonus_list{
                match bonus_event{
                    BonusEvent::NewPlayer(player_id) => {
                        let new_player = NewPlayerInfo{
                            player_id: *player_id,
                            frame_added: self.start_frame + package_relative_frame_index,
                        };
                        new_players.push(new_player);
                    }
                }
            }
        }
        return new_players;
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncerData<T> {
    pub data: Vec<T>,
    pub start_frame: FrameIndex,
    pub owning_player: i32// Unused for bonus msgs. i32 not usize as unused is -1.
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SyncerRequestType {
    PlayerInputs(PlayerID),
    BonusEvents,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncerRequestTyped {
    pub request: SyncerRequest,
    pub type_needed: SyncerRequestType,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncerRequest {
    pub start_frame: FrameIndex,
    pub number_of_frames: usize, // Usually 20.
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncerStore<T> {
    pub frames_index_offset: usize, // This is needed as this might all be used for a player who can join midway.
    pub data: Vec<T>,
}

impl<T> SyncerStore<T> where T: Clone{
    pub fn gen_bonus_store(frames_index_offset: usize) -> SyncerStore<Vec<BonusEvent>>{
        return SyncerStore{
            frames_index_offset,
            data: vec![]
        }
    }
    pub fn gen_inputs_store(frames_index_offset: usize) -> SyncerStore<InputState>{
        return SyncerStore{
            frames_index_offset,
            data: vec![]
        }
    }
    pub fn get_single_item(&self, frame_index: FrameIndex) -> Option<&T> {
        if frame_index < self.frames_index_offset{
            // Since lists of player inputs are inited when players join, we can sometimes try to get info from frames before 0.
            panic!("NOT ALLOWED TO QUERY INFORMATION FROM BEFORE PLAYER IS INITIALIZED.");
        }
        return self.data.get(frame_index - self.frames_index_offset);
    }
    pub fn get_or_last_query(&self, frame_index: FrameIndex, request_type: SyncerRequestType) -> (Option<T>, Option<SyncerRequestTyped>){
        // Returns data if found at index or found at vec end.
        // Returns typed error if not found at index.

        let data_option = self.get_single_item(frame_index);
        let data;
        let mut missing = None;
        match data_option{
            Some(found_data) => {
                data = Some(found_data.clone());
            }
            None => {
                missing = Some(SyncerRequestTyped{
                    request: SyncerRequest {
                        start_frame: frame_index,
                        number_of_frames: 20 // modival
                    },
                    type_needed: request_type
                });
                data = self.get_last().cloned();
            }
        }
        return (data, missing);
    }
    pub fn get_last(&self) -> Option<&T>{
        return self.data.last();
    }
    pub fn get_data_segment(&self, request: &SyncerRequestTyped) -> SyncerData<T> { // This is used when server responds to client's missing input request.
        // Here we're assuming that the reqest is of the correct type.
        let relative_start_frame = request.request.start_frame - self.frames_index_offset;

        let mut data_found = vec![];
        'outer: for relative_index in relative_start_frame..(relative_start_frame + request.request.number_of_frames /*No need for +1 */){
            let data_item = self.data.get(relative_index);
            match data_item{
                Some(item) => {
                    data_found.push(item.clone());
                }
                None => {
                    break 'outer;
                }
            }
        }
        let mut player_id = -1;
        match request.type_needed{
            SyncerRequestType::PlayerInputs(id) => {
                player_id = id as i32;
            }
            _ => {}
        }
        return SyncerData{
            data: data_found,
            start_frame: request.request.start_frame,
            owning_player: player_id
        }
    }
    pub fn insert_data_segment(&mut self, syncer_data: SyncerData<T>){

        for (input_vec_index, item) in syncer_data.data.iter().enumerate(){
            let absolute_index = syncer_data.start_frame + input_vec_index;
            let relative_index = absolute_index - self.frames_index_offset;

            vec_replace_or_end(&mut self.data,relative_index, item.clone()); // Pointless_optimum no need to clone.
        }
    }
}
























