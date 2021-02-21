use std::net::{SocketAddr, TcpStream};
use std::ops::Add;
use std::str::FromStr;
use crossbeam_channel::*;
use std::thread;
use std::time::{Duration, SystemTime};
use std::ops::Div;
use std::ops::Sub;
use serde::{Deserialize, Serialize};
use crate::netcode::common::network::external_msg::*;
use std::sync::{RwLock, Arc, RwLockWriteGuard, Mutex};
use std::collections::vec_deque::*;
use std::io::Seek;
use crate::netcode::client::input_handler_seg::*;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use crate::netcode::common::utils::util_functions::vec_replace_or_end;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SuperstoreData<T> {
    pub data: Vec<T>,
    pub frame_offset: FrameIndex
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Superstore<T:Clone + Default + Send +  Eq + std::fmt::Debug + Sync + 'static>{
    frame_offset: usize,
    data: Vec<T>,
}


impl<T:Clone + Default + Send +  Eq + std::fmt::Debug + Sync + 'static> Superstore<T>{
    pub fn new(first_frame_to_store: FrameIndex) -> Self{
        Self{
            frame_offset: first_frame_to_store,
            data: vec![]
        }
    }
    pub fn get(&self, abs_index: FrameIndex) -> Option<&T>{
        if abs_index < self.frame_offset{
            return None;
        }
        return self.data.get(abs_index - self.frame_offset);
    }
    pub fn get_last(&self) -> Option<&T>{
        return self.data.last();
    }
    pub fn clone_block(&self, first_frame_index: FrameIndex, block_size: usize) -> Vec<T>{
        let mut query_response = vec![];

        for target_index in first_frame_index..(first_frame_index + block_size){
            match self.get(target_index){
                Some(input) => {
                    query_response.push(input);
                }
                None => {
                    break;
                }
            }
        };
        return query_response;
    }
    pub fn write_data(&mut self, new_data: SuperstoreData<T>){

        // If new's first frame is in existing data, or the next new frame.
        if new_data.frame_offset >= self.frame_offset && new_data.frame_offset <= 1 + self.frame_offset + self.data.len(){
            for (new_relative_index, new_item) in new_data.data.into_iter().enumerate(){
                let existing_relative_index = new_data.frame_offset + new_relative_index - self.frame_offset;

                vec_replace_or_end(&mut self.data, existing_relative_index, new_item);
            }
        }
        // If new data is behind existing data, but no gap.
        else if new_data.frame_offset + new_data.data.len() >= self.data.len(){
            let number_behind_count = self.frame_offset - new_data.frame_offset;
            let overlap_count = new_data.frame_offset + new_data.data.len() - self.frame_offset;

            self.frame_offset = new_data.frame_offset;

            self.data.drain(0..overlap_count);

        }else{
            log::info!("Received data would make a gap. Ignoring."); // Can change the 'info' to 'trace'.
        }
    }

}