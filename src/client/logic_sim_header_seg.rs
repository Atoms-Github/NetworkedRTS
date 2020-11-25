use std::thread;
use std::sync::{Arc, RwLock};
use crossbeam_channel::*;

use crate::common::gameplay::game::game_state::*;
use crate::common::sim_data::sim_data_storage::*;
use crate::common::time::timekeeping::*;
use crate::common::types::ArcRw;

pub const HEAD_AHEAD_FRAME_COUNT: usize = 20;


pub struct LogicSimHeaderIn {
    known_frame_info: KnownFrameInfo,
    tail_rec: Receiver<GameState>,
    data_store: SimDataStorageEx,
}
pub struct LogicSimHeaderEx {
    pub head_rec: Option<Receiver<GameState>>,
}


fn deep_clone_state_lock(state_tail: &ArcRw<GameState>) -> ArcRw<GameState>{
    let guard = state_tail.read().unwrap();
    let head_state = (*guard).clone();
    Arc::new(RwLock::new(head_state))
}

impl LogicSimHeaderIn {
    pub fn new(known_frame_info: KnownFrameInfo, tail_rec: Receiver<GameState>,
               data_store: SimDataStorageEx)
               -> LogicSimHeaderIn {
        LogicSimHeaderIn {
            known_frame_info,
            data_store,
            tail_rec
        }
    }

    pub fn start(mut self) -> LogicSimHeaderEx{
        let (mut head_sink, mut head_rec) = unbounded();
        thread::spawn(move ||{
            loop{
                let tail = self.tail_rec.recv().unwrap();
//                println!("Head got frame {}", tail.get_simmed_frame_index());

                let new_head = self.calculate_new_head(tail);
//                println!("Head sending {}", new_head.get_simmed_frame_index());
                head_sink.send(new_head).unwrap();

            }
        });

        LogicSimHeaderEx{
            head_rec: Some(head_rec)
        }
    }
    fn calculate_new_head(&mut self, mut state_tail: GameState) -> GameState{
        let first_head_to_sim = state_tail.get_simmed_frame_index() + 1;
        let frames_to_sim_range = first_head_to_sim..(first_head_to_sim + HEAD_AHEAD_FRAME_COUNT);

        let mut infos_for_sims = vec![];
        { // Get all information needed from frames database.
//            println!("{:?}", all_frames);
            for frame_index in frames_to_sim_range{
                infos_for_sims.push(self.data_store.clone_info_for_head(frame_index));
            }
        } // Discard frame info database lock.

//        println!("Simming head with: {:?}", infos_for_sims);
        for sim_info in infos_for_sims{
            state_tail.simulate_tick(sim_info, FRAME_DURATION_MILLIS);
        }
        state_tail
    }
}

