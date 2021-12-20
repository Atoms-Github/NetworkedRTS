use crate::pub_types::{FrameIndex, HashType};
use std::collections::HashMap;
use serde::*;
use crate::common::net_game_state::NetGameState;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
}