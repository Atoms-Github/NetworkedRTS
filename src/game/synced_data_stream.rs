
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{SystemTime};

use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_hub_segment::{DistributableNetMessage, NetworkingHub, OwnedNetworkMessage};
use crate::network::networking_structs::*;
use crate::network::networking_message_types::*;
use crate::network::game_message_types::*;
use std::sync::{Mutex, Arc};
use std::thread;
use std::panic;
use crate::game::timekeeping::*;
use crate::network::networking_structs::*;
use crate::network::game_message_types::*;
use std::collections::HashMap;
use std::thread::Thread;
use std::time::Duration;
use crate::players::inputs::*;
use serde::{Deserialize, Serialize};
use crate::utils::util_functions::*;
use crate::game::bonus_msgs_segment::*;
use crate::players::inputs::*;


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
    pub head_ahead_frames: i32,
    pub data: Vec<T>,
}

impl<T> SyncerStore<T> where T: Clone{
    pub fn gen_bonus_store(frames_index_offset: usize) -> SyncerStore<Vec<BonusEvent>>{
        return SyncerStore{
            frames_index_offset,
            head_ahead_frames: 60,
            data: vec![]
        }
    }
    pub fn gen_inputs_store(frames_index_offset: usize) -> SyncerStore<InputState>{
        return SyncerStore{
            frames_index_offset,
            head_ahead_frames: 20,
            data: vec![]
        }
    }
    pub fn get_single_item(&self, frame_index: FrameIndex) -> Option<&T> {
        return self.data.get(frame_index - self.frames_index_offset);
    }
    pub fn get_frames_segment(&self, request: &SyncerRequestTyped) -> SyncerData<T> { // This is used when server responds to client's missing input request.
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
        let mut playerId = -1;
        match request.type_needed{
            SyncerRequestType::PlayerInputs(id) => {
                playerId = id as i32;
            }
            _ => {}
        }
        return SyncerData{
            data: data_found,
            start_frame: request.request.start_frame,
            owning_player: playerId
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
























