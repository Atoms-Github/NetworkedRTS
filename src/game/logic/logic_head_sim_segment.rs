use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, TryRecvError, Sender};
use serde::{Deserialize, Serialize};

use crate::game::timekeeping::*;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_structs::*;
use std::{panic, thread};
use std::collections::HashMap;
use std::time::Duration;
use crate::game::synced_data_stream::*;
use crate::players::inputs::*;
use crate::game::bonus_msgs_segment::*;
use crate::game::logic::logic_data_storage::*;

pub const HEAD_AHEAD_FRAME_COUNT: usize = 20;


pub struct LogicHeadSimIn {
    known_frame_info: KnownFrameInfo,
    head_lock: Arc<RwLock<GameState>>,
    tail_lock: Arc<RwLock<GameState>>,
    all_frames: Arc<RwLock<LogicDataStorage>>,
}
pub struct LogicHeadSimEx {
    pub head_lock: Arc<RwLock<GameState>>,
}

fn deep_clone_state_lock(state_tail: &Arc<RwLock<GameState>>) -> Arc<RwLock<GameState>>{
    let guard = state_tail.read().unwrap();
    let head_state = (*guard).clone();
    return Arc::new(RwLock::new(head_state));
}

impl LogicHeadSimIn {
    pub fn new(known_frame_info: KnownFrameInfo, tail_lock: Arc<RwLock<GameState>>,
               data_store: Arc<RwLock<LogicDataStorage>>) // TODO2: Refactor arguments.
               -> LogicHeadSimIn {
        return LogicHeadSimIn {
            known_frame_info,
            head_lock: deep_clone_state_lock(&tail_lock),
            tail_lock,
            all_frames: data_store
        };
    }


    fn set_new_head(&mut self, new_head: GameState){
        *self.head_lock.write().unwrap() = new_head;
    }

    pub fn start(mut self) -> LogicHeadSimEx{
        let head_handle = self.head_lock.clone();
        thread::spawn(move ||{
            let mut my_self = self;
            let start_frame = my_self.tail_lock.read().unwrap().get_simmed_frame_index();
            let generator = my_self.known_frame_info.start_frame_stream_from_any(start_frame);
            loop{
                generator.recv().unwrap();
                let tail = my_self.clone_tail();
                let new_head = my_self.calculate_new_head(tail);
                my_self.set_new_head(new_head);
            }
        });

        return LogicHeadSimEx{
            head_lock: head_handle
        };
    }

    fn clone_tail(&self) -> GameState{
        return self.tail_lock.read().unwrap().clone();
    }
    fn calculate_new_head(&mut self, mut state_tail: GameState) -> GameState{
        let first_head_to_sim = state_tail.get_simmed_frame_index() + 1;
        let frames_to_sim_range = first_head_to_sim..(first_head_to_sim + HEAD_AHEAD_FRAME_COUNT);

        let mut infos_for_sims = vec![];
        { // Get all information needed from frames database.
            let all_frames = self.all_frames.read().unwrap();
            for frame_index in frames_to_sim_range{
                let clone_data_result = all_frames.clone_info_for_sim(frame_index);
                // We don't care about the failures - we want any info we can get.
                infos_for_sims.push(clone_data_result.sim_info);
            }
        } // Discard frame info database lock.

        for sim_info in infos_for_sims{
            state_tail.simulate_tick(&sim_info, FRAME_DURATION_MILLIS);
        }
        return state_tail;
    }
}

