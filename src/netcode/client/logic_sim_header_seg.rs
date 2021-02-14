use std::thread;
use std::sync::{Arc, RwLock};
use crossbeam_channel::*;

use crate::netcode::*;
use crate::netcode::common::sim_data::sim_data_storage::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use crate::netcode::common::sim_data::net_game_state::{NetPlayerProperty, NetGameState};

pub const HEAD_AHEAD_FRAME_COUNT: usize = 20;


pub struct LogicSimHeaderIn {
    known_frame_info: KnownFrameInfo,
    tail_rec: Receiver<NetGameState>,
    data_store: SimDataStorageEx,
}
pub struct LogicSimHeaderEx {
    pub head_rec: Option<Receiver<NetGameState>>,
}
impl LogicSimHeaderEx{
    pub fn start(known_frame_info: KnownFrameInfo, tail_rec: Receiver<NetGameState>, data_store: SimDataStorageEx) -> Self {
        LogicSimHeaderIn {
            known_frame_info,
            data_store,
            tail_rec
        }.start()
    }
}


fn deep_clone_state_lock(state_tail: &ArcRw<NetGameState>) -> ArcRw<NetGameState>{
    let guard = state_tail.read().unwrap();
    let head_state = (*guard).clone();
    Arc::new(RwLock::new(head_state))
}

impl LogicSimHeaderIn {


    pub fn start(mut self) -> LogicSimHeaderEx{
        let (mut head_sink, mut head_rec) = unbounded();
        thread::spawn(move ||{
            loop{
                let tail = self.tail_rec.recv().unwrap();
                log::trace!("Head got frame {}", tail.get_simmed_frame_index());

                let new_head = self.calculate_new_head(tail);
                log::trace!("Head sending {}", new_head.get_simmed_frame_index());
                head_sink.send(new_head).unwrap();

            }
        });

        LogicSimHeaderEx{
            head_rec: Some(head_rec)
        }
    }
    fn calculate_new_head(&mut self, mut state_tail: NetGameState) -> NetGameState {
        let first_head_to_sim = state_tail.get_simmed_frame_index() + 1;
        let frames_to_sim_range = first_head_to_sim..(first_head_to_sim + HEAD_AHEAD_FRAME_COUNT);

        let mut infos_for_sims = vec![];
        { // Get all information needed from frames database.
            for frame_index in frames_to_sim_range{
                infos_for_sims.push(self.data_store.clone_info_for_head(frame_index));
            }
        } // Discard frame info database lock.

        log::trace!("Simming head with: {:?}", infos_for_sims);
        for sim_info in infos_for_sims{
            state_tail.simulate_tick(sim_info, FRAME_DURATION_MILLIS);
        }
        state_tail
    }
}
