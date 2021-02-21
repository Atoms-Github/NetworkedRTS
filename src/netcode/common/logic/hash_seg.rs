
use crossbeam_channel::*;
use std::thread;
use serde::*;
use crate::netcode::client::logic_sim_header_seg::*;
use crate::netcode::common::logic::logic_sim_tailer_seg::*;
use crate::netcode::common::sim_data::input_state::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::common::sim_data::sim_data_storage::*;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use std::collections::HashMap;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FramedHash{
    pub frame: FrameIndex,
    pub hash: HashType,
}
#[derive(Clone)]
pub struct HasherEx {
    hashes: HashMap<FrameIndex, HashType>, // pointless_optimum Could use vec, but easier to use hashmap.
}
impl HasherEx {
    pub fn new() -> Self{
        Self{
            hashes: Default::default()
        }
    }
    pub fn add_hash(&mut self, framed_hash: FramedHash){
        match self.hashes.get(&framed_hash.frame){
            None => {
                self.hashes.insert(framed_hash.frame, framed_hash.hash);
            }
            Some(existing_hash) => {
                assert!(*existing_hash == framed_hash.hash, format!("Out of sync! Frame index {}", framed_hash.frame));
            }
        }
    }

}