use serde::{Deserialize, Serialize};

use crate::netcode::common::types::*;
use crate::netcode::common::utils::util_functions::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FramedVecDataPack<T> {
    pub data: Vec<T>,
    pub start_frame: FrameIndex,
    pub owning_player: PlayerID
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FramedVecRequestType {
    PlayerInputs(PlayerID),
    BonusEvents,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FramedVecRequestTyped {
    pub request: FramedVecRequest,
    pub type_needed: FramedVecRequestType,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FramedVecRequest {
    pub start_frame: FrameIndex,
    pub number_of_frames: usize, // Usually 20.
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FramedVec<T> {
    pub frames_index_offset: usize, // This is needed as this might all be used for a player who can join midway.
    pub data: Vec<T>,
}

impl<T> FramedVec<T> where T: Clone{
    pub fn new(frames_index_offset: usize) -> FramedVec<T>{
        FramedVec{
            frames_index_offset,
            data: vec![]
        }
    }
    pub fn get_single_item(&self, frame_index: FrameIndex) -> Option<&T> {
        if frame_index < self.frames_index_offset{
            // Since lists of player sync are inited when players join, we can sometimes try to get info from frames before 0.
            panic!("NOT ALLOWED TO QUERY INFORMATION FROM BEFORE PLAYER IS INITIALIZED.");
        }
        self.data.get(frame_index - self.frames_index_offset)
    }
    pub fn get_or_last_query(&self, frame_index: FrameIndex, request_type: FramedVecRequestType) -> (Option<T>, Option<FramedVecRequestTyped>){
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
                missing = Some(FramedVecRequestTyped{
                    request: FramedVecRequest {
                        start_frame: frame_index,
                        number_of_frames: 10 // modival
                    },
                    type_needed: request_type
                });
                data = self.get_last().cloned();
            }
        }
        (data, missing)
    }
    pub fn get_last(&self) -> Option<&T>{
        self.data.last()
    }
    pub fn get_data_segment(&self, request: &FramedVecRequestTyped) -> FramedVecDataPack<T> { // This is used when server responds to client's missing input request.
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
        match request.type_needed{
            FramedVecRequestType::PlayerInputs(id) => {
                FramedVecDataPack{
                    data: data_found,
                    start_frame: request.request.start_frame,
                    owning_player: id
                }
            }
            _ => {
                panic!("Shouldn't be any other types.");
            }
        }

    }
    pub fn insert_data_segment(&mut self, syncer_data: FramedVecDataPack<T>){

        for (input_vec_index, item) in syncer_data.data.iter().enumerate(){
            let absolute_index = syncer_data.start_frame + input_vec_index;
            if self.frames_index_offset > absolute_index{
                panic!("Tried to insert data into segment from earler than segment was collecting data from. Collecting from {}. Insert at {}", self.frames_index_offset, absolute_index);
            }
            let relative_index = absolute_index - self.frames_index_offset;

            vec_replace_or_end(&mut self.data,relative_index, item.clone()); // Pointless_optimum no need to clone.
        }
    }
}
























