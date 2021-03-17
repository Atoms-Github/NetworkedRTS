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
pub struct Superstore<T:Clone + Default + Send +  std::fmt::Debug + Sync + 'static>{
    frame_offset: usize,
    data: Vec<T>,
    waiting_data: Vec<SuperstoreData<T>>, // All the future data which would make a hole. This is held until the hole is filled.
}




impl<T:Clone + Default + Send +  std::fmt::Debug + Sync + 'static> Superstore<T>{
    pub fn new(first_frame_to_store: FrameIndex) -> Self{
        Self{
            frame_offset: first_frame_to_store,
            data: vec![],
            waiting_data: vec![]
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
    pub fn get_next_empty_frame(&self) -> FrameIndex{
        return self.data.len() + self.frame_offset;
    }
    pub fn clone_block(&self, first_frame_index_abs: FrameIndex, block_size: usize) -> Vec<T>{
        let mut query_response = vec![];

        let first_frame_index_rel = first_frame_index_abs - self.frame_offset;
        for target_index_relative in first_frame_index_rel..(first_frame_index_rel + block_size){
            match self.get(target_index_relative){
                Some(input) => {
                    query_response.push(input.clone());
                }
                None => {
                    break;
                }
            }
        };
        return query_response;
    }
    pub fn write_data(&mut self, new_data: SuperstoreData<T>){
        // If future data. (So far ahead it would leave a gap.
        if new_data.frame_offset > self.get_next_empty_frame() {
            // Save future data for later.
            self.waiting_data.push(new_data);
            //panic!("Received player data for the future, so distant we can't handle it. Incoming data first frame: {}. We're waiting on frame {}", new_data.frame_offset, self.get_next_empty_frame());
        }
        // If new's first frame is in existing data, or the next new frame.
        else if new_data.frame_offset >= self.frame_offset && new_data.frame_offset <= self.get_next_empty_frame(){
            for (new_relative_index, new_item) in new_data.data.into_iter().enumerate(){
                let existing_relative_index = new_data.frame_offset + new_relative_index - self.frame_offset;
                vec_replace_or_end(&mut self.data, existing_relative_index, new_item);
            }
            // pointless_optimum don't need to move out.
            // TODO3: Find a nicer way of writing this.
            let mut items = vec![];
            let waiting_items = self.waiting_data.drain(..).for_each(|item|{items.push(item)});
            for waiting_item in items{
                self.write_data(waiting_item);
            }
        }

        // Need if statement here!
        // If new data is behind existing data, but no gap.
        // else if new_data.frame_offset + new_data.data.len() >= self.frame_offset{
        //     let number_behind_count = self.frame_offset - new_data.frame_offset;
        //     let overlap_count = new_data.frame_offset + new_data.data.len() - self.frame_offset;
        //
        //     println!("Shifting backwards! -- {}, {}, {}", new_data.frame_offset, new_data.data.len(), self.frame_offset);
        //     self.frame_offset = new_data.frame_offset;
        //
        //     self.data.drain(0..overlap_count);
        /*}*/else{
            log::info!("Got some early data. We couldn't care less about this data. We start {}, we have {} items Data is {:?}", self.frame_offset, self.data.len(), new_data);
            // panic!("Known issue #1. Somehow recieved late player data, then early player data which would leave a hole. Can be fixed by using a hashmap to store all inputs. As of now, we're trusting clients to send inputs in a reasonable order.");
        }
    }

}
