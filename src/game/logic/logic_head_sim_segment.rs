use std::sync::{Arc, RwLock};

use crate::game::timekeeping::*;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_structs::*;
use std::{thread};
use crate::game::logic::logic_data_storage::*;
use std::sync::mpsc::{Receiver, channel};

pub const HEAD_AHEAD_FRAME_COUNT: usize = 20;


pub struct LogicHeadSimIn {
    known_frame_info: KnownFrameInfo,
    tail_rec: Receiver<GameState>,
    all_frames: Arc<RwLock<LogicDataStorage>>,
}
pub struct LogicHeadSimEx {
    pub head_rec: Receiver<GameState>,
}

fn deep_clone_state_lock(state_tail: &Arc<RwLock<GameState>>) -> Arc<RwLock<GameState>>{
    let guard = state_tail.read().unwrap();
    let head_state = (*guard).clone();
    return Arc::new(RwLock::new(head_state));
}

impl LogicHeadSimIn {
    pub fn new(known_frame_info: KnownFrameInfo, tail_rec: Receiver<GameState>,
               data_store: Arc<RwLock<LogicDataStorage>>) // TODO2: Refactor arguments.
               -> LogicHeadSimIn {
        return LogicHeadSimIn {
            known_frame_info,
            all_frames: data_store,
            tail_rec
        };
    }



    pub fn start(mut self) -> LogicHeadSimEx{
        let (mut head_sink, mut head_rec) = channel();
        thread::spawn(move ||{
            loop{
                let tail = self.tail_rec.recv().unwrap();
                println!("Head got frame {}", tail.get_simmed_frame_index());

                let new_head = self.calculate_new_head(tail);
                println!("Head sending {}", new_head.get_simmed_frame_index());
                head_sink.send(new_head).unwrap();

            }
        });

        return LogicHeadSimEx{
            head_rec
        };
    }

//    fn clone_tail(&self) -> GameState{
//        return self.tail_lock.read().unwrap().clone();
//    }
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
            state_tail.simulate_tick(sim_info, FRAME_DURATION_MILLIS);
        }
        return state_tail;
    }
}

