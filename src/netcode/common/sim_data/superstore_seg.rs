use std::net::{SocketAddr, TcpStream};
use std::ops::{Add, RangeFrom};
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
use std::collections::BTreeMap;
use crate::netcode::common::sim_data::superstore_seg::Cap::Pioneer;
use std::collections::btree_map::Range;
use std::fmt::Debug;
use crate::netcode::common::sim_data::confirmed_data::JoinType;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SuperstoreData<T> {
    pub data: Vec<T>,
    pub frame_offset: FrameIndex
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Superstore<T :Clone + Default + Send + Debug + Sync + 'static>{
    confirmed_data: bool,
    data: BTreeMap<FrameIndex, Vec<T>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub enum StoreItem<T :Clone + Default + Send + Debug + Sync + 'static>{
    Data(T),
    Cap(Cap)
}
#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub enum Cap{
    Connect(JoinType), // Replaces first input. I.e. first input is blank.
    Disconnect, // Frame after last input.
}
enum Direc{
    Forwards,
    Backwards,
}


impl<T:Clone + Default + Send +  std::fmt::Debug + Sync + 'static> Superstore<T>{
    pub fn new(confirmed_data: bool) -> Self{
        Self{
            confirmed_data,
            data: Default::default()
        }
    }
    pub fn get(&self, index: FrameIndex) -> Option<&T>{
        // If direct hit:
        if let Some(my_data) = self.data.get(&index){
            return my_data.first();
        }
        let previous_maybe = self.get_closest(index, Direc::Backwards)?;


        let (frame, data) = self.data.range(..index).next().unwrap();

        // Breaking.
        return None;
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
    fn get_closest(&self, frame_index: FrameIndex, direction: Direc) -> Option<(&FrameIndex, &Vec<T>)>{
        match direction{
            Direc::Forwards => {
                self.data.range(frame_index..).next() // breaking +1 / -1? https://stackoverflow.com/questions/49599833/how-to-find-next-smaller-key-in-btreemap-btreeset
            }
            Direc::Backwards => {
                self.data.range(..frame_index).next()
            }
        }

    }
    pub fn write_data(&mut self, new_data: SuperstoreData<T>){
        // If can just plonk with no overlap:
        // breaking.

    }

}
