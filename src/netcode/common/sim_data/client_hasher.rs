use crate::pub_types::{FrameIndex, HashType};
use std::collections::HashMap;
use crate::netcode::common::simulation::net_game_state::NetGameState;
use serde::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FramedHash{
    pub frame: FrameIndex,
    pub hash: HashType,
}

pub struct ClientHasher{
    hashes: HashMap<FrameIndex, HashType>
}
impl ClientHasher{
    pub fn new() -> Self{
        Self{
            hashes: Default::default(),
        }
    }
    pub fn add_framed(&mut self, framed_hash: FramedHash){
        if let Some(existing_hash) = self.hashes.insert(framed_hash.frame, framed_hash.hash){
            assert_eq!(existing_hash, framed_hash.hash, "Out of sync! Frame index {}", framed_hash.frame);
        }
    }
    pub fn add_state(&mut self, state: &NetGameState){
        self.add_framed(FramedHash{
            frame: state.get_simmed_frame_index(),
            hash: state.get_hash(),
        });
    }
}