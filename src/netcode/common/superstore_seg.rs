


use std::net::{SocketAddr, TcpStream};
use std::ops::{Add, RangeFrom};
use std::str::FromStr;
use crossbeam_channel::*;
use std::thread;
use std::time::{Duration, SystemTime};
use std::ops::Div;
use std::ops::Sub;
use serde::{Deserialize, Serialize};
use crate::netcode::common::external_msg::*;
use std::sync::{RwLock, Arc, RwLockWriteGuard, Mutex};
use std::collections::vec_deque::*;
use std::io::Seek;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use std::collections::{BTreeMap, HashMap};
use std::collections::btree_map::Range;
use std::fmt::Debug;
use mopa::Any;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SuperstoreData<T> {
    pub data: Vec<T>,
    pub frame_offset: FrameIndex
}
impl<T> SuperstoreData<T>{
    pub fn trim_earlier(&self, earliest_frame: FrameIndex) -> SuperstoreData<T>{
        // Yes, this could be optimised.
        let mut output = vec![];
        for (relative, item) in self.data.into_iter().enumerate(){
            let abs_index = relative + self.frame_offset;
            if abs_index >= earliest_frame{
                output.push(item)
            }
        }
        SuperstoreData{
            data: output,
            frame_offset: earliest_frame.max(self.frame_offset)
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Superstore<T :Clone + Default + Send + Debug + Sync + PartialEq + 'static>{
    confirmed_data: bool,
    data: HashMap<FrameIndex, T>,
    pub last_frame: Option<FrameIndex>,
}


impl<T:Clone + Default + Send +  std::fmt::Debug + Sync + PartialEq + 'static> Superstore<T>{
    pub fn new(confirmed_data: bool) -> Self{
        Self{
            confirmed_data,
            data: Default::default(),
            last_frame: None
        }
    }
    pub fn get(&self, index: FrameIndex) -> Option<&T>{
        return self.data.get(&index);
    }
    pub fn clone_block(&self, first_frame: FrameIndex, block_size: usize) -> Vec<T>{
        let mut query_response = vec![];
        for target_index in first_frame..(first_frame + block_size){
            match self.get(target_index){
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
        for (i, item) in new_data.data.into_iter().enumerate(){
            let abs_index = i + new_data.frame_offset;
            // Clone is better than two hashmap lookups.
            let existing = self.data.insert(abs_index, item.clone());
            if self.confirmed_data && existing.is_some(){
                assert_eq!(existing.unwrap(), item, "Data should've been confirmed!");
            }
            if let Some(last_frame) = &mut self.last_frame{
                *last_frame = (*last_frame).max(abs_index);
            }
        }
    }
}







// use std::net::{SocketAddr, TcpStream};
// use std::ops::{Add, RangeFrom};
// use std::str::FromStr;
// use crossbeam_channel::*;
// use std::thread;
// use std::time::{Duration, SystemTime};
// use std::ops::Div;
// use std::ops::Sub;
// use serde::{Deserialize, Serialize};
// use crate::netcode::common::network::external_msg::*;
// use std::sync::{RwLock, Arc, RwLockWriteGuard, Mutex};
// use std::collections::vec_deque::*;
// use std::io::Seek;
// use crate::netcode::client::input_handler_seg::*;
// use crate::netcode::netcode_types::*;
// use crate::pub_types::*;
// use crate::netcode::common::utils::util_functions::vec_replace_or_end;
// use std::collections::BTreeMap;
// use std::collections::btree_map::Range;
// use std::fmt::Debug;
// use crate::netcode::common::sim_data::confirmed_data::JoinType;
// use mopa::Any;
//
//
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct SuperstoreData<T> {
//     pub data: Vec<T>,
//     pub frame_offset: FrameIndex
// }
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Superstore<T :Clone + Default + Send + Debug + Sync + 'static>{
//     confirmed_data: bool,
//     data: BTreeMap<FrameIndex, Vec<T>>,
// }
//
//
// enum Direc{
//     Forwards,
//     Backwards,
// }
//
//
// impl<T:Clone + Default + Send +  std::fmt::Debug + Sync + 'static> Superstore<T>{
//     pub fn new(confirmed_data: bool) -> Self{
//         Self{
//             confirmed_data,
//             data: Default::default()
//         }
//     }
//     pub fn get(&self, index: FrameIndex) -> Option<&T>{
//         let (prev_frame, prev_data) = self.get_closest(index, Direc::Backwards)?;
//         let relative_index = index - *prev_frame;
//         return prev_data.get(relative_index);
//     }
//     pub fn clone_block(&self, first_frame: FrameIndex, block_size: usize) -> Vec<T>{
//         let mut query_response = vec![];
//         for target_index in first_frame..(first_frame + block_size){
//             match self.get(target_index){
//                 Some(input) => {
//                     query_response.push(input.clone());
//                 }
//                 None => {
//                     break;
//                 }
//             }
//         };
//         return query_response;
//     }
//     fn get_closest(&self, frame_index: FrameIndex, direction: Direc) -> Option<(&FrameIndex, &Vec<T>)>{
//         match direction{
//             Direc::Forwards => {
//                 self.data.range(frame_index..).next() // breaking Need to allow direct hits. +1 / -1? https://stackoverflow.com/questions/49599833/how-to-find-next-smaller-key-in-btreemap-btreeset
//             }
//             Direc::Backwards => {
//                 self.data.range(..frame_index).next()
//             }
//         }
//     }
//     fn try_merge(&mut self, back_index: FrameIndex, front_index: FrameIndex){
//
//     }
//     fn add_data(&mut self, existing_data: FrameIndex, new_data: SuperstoreData<T>){
//
//     }
//     pub fn write_data(&mut self, new_data: SuperstoreData<T>){
//         // What we want to do:
//         // If backwards in range:
//         //      Move all into backwards.
//         //      Try to merge forward and backwards.
//         // Else if forwards in range:
//         //      Move all into forward.
//         // Else:
//         //      Move into blank space.
//         //
//
//         if let Some((prev_frame, prev_data)) =
//         self.get_closest(new_data.frame_offset, Direc::Backwards){
//             let last_included_index = *prev_frame + prev_data.len();
//             if new_data.frame_offset <= last_included_index + 1{
//
//             }
//         }
//
//         // If can just plonk with no overlap:
//         // breaking.
//
//     }
// }

// #[derive(Clone, Serialize, Deserialize, Debug, Hash)]
// pub enum StoreItem<T :Clone + Default + Send + Debug + Sync + 'static>{
//     Data(T),
//     Cap(Cap)
// }
// #[derive(Clone, Serialize, Deserialize, Debug, Hash)]
// pub enum Cap{
//     Connect(JoinType), // Replaces first input. I.e. first input is blank.
//     Disconnect, // Frame after last input.
// }