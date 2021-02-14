
use crossbeam_channel::*;
use std::thread;
use serde::*;
use crate::netcode::client::logic_sim_header_seg::*;
use crate::netcode::common::logic::logic_sim_tailer_seg::*;
use crate::netcode::common::sim_data::framed_vec::*;
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
impl FramedHash{
    pub fn new(frame: FrameIndex, hash: HashType) -> Self{
        Self{
            frame,
            hash,

        }
    }
}

#[derive(Clone)]
pub struct HasherEx {
    hashes_sink: Sender<FramedHash>,
}
impl HasherEx {
    pub fn add_hash(&self, framed_hash: FramedHash){
        self.hashes_sink.send(framed_hash).unwrap();
    }
    pub fn link_hash_stream(&self, stream: Receiver<FramedHash>){ // pointless_optimum In a perfect world, this shouldn't be needed.
        let sink = self.hashes_sink.clone();
        thread::spawn(move||{
            loop{
                sink.send(stream.recv().unwrap()).unwrap();
            }
        });
    }
    pub fn start() -> Self{
        let (hashes_sink, hashes_rec) = unbounded();
        let hasher_in = HasherIn{
            hashes: HashMap::default(),
            hashes_rec
        };
        hasher_in.start_thread();
        HasherEx{
            hashes_sink
        }
    }
}
pub struct HasherIn {
    hashes: HashMap<FrameIndex, HashType>, // pointless_optimum Could use vec, but easier to use hashmap.
    hashes_rec: Receiver<FramedHash>
}
impl HasherIn {
    fn start_thread(mut self){
        thread::spawn(move ||{
            loop{
                let framed_hash = self.hashes_rec.recv().unwrap();
                match self.hashes.get(&framed_hash.frame){
                    None => {
                        self.hashes.insert(framed_hash.frame, framed_hash.hash);
                    }
                    Some(existing_hash) => {
                        assert!(*existing_hash == framed_hash.hash, format!("Out of sync! Frame index {}", framed_hash.frame));
                    }
                }
            }
        });
    }
}

