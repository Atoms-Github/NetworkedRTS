use std::thread;
use std::sync::{Arc, RwLock};
use crossbeam_channel::*;

use serde::{Deserialize, Serialize};

use crate::common::gameplay::game::game_state::*;
use crate::common::sim_data::framed_vec::*;
use crate::common::sim_data::input_state::*;
use crate::common::time::timekeeping::*;
use crate::common::types::*;

use crate::common::sim_data::superstore_seg::*;
use crate::common::sim_data::sim_data_storage::*;
use std::time::{SystemTime, Duration};

use crate::common::data::hash_seg::*;
use std::hash::Hash;

pub struct LogicSimTailerEx {
    pub from_logic_rec: Receiver<QuerySimData>,
    pub tail_lock: ArcRw<GameState>,
    pub new_tail_states_rec: Option<Receiver<GameState>>,
    pub new_tail_hashes: Option<Receiver<FramedHash>>,

}
impl LogicSimTailerEx {

}
pub struct LogicSegmentTailerIn {
    known_frame_info: KnownFrameInfo,
    tail_lock: ArcRw<GameState>,
    data_store: SimDataStorageEx
    // Logic layer shouldn't know it's player ID.
}

impl LogicSegmentTailerIn {
    pub fn new(known_frame_info: KnownFrameInfo, state_tail: GameState,
               data_store: SimDataStorageEx) -> LogicSegmentTailerIn {
        LogicSegmentTailerIn {
            known_frame_info,
            tail_lock: Arc::new(RwLock::new(state_tail)),
            data_store
        }
    }

    fn try_sim_tail_frame(&mut self, tail_frame_to_sim: FrameIndex) -> Vec<QuerySimData>{
        let sim_query_result = self.data_store.clone_info_for_tail(tail_frame_to_sim);
        match sim_query_result{
            Ok(sim_info) => {
                let mut state_handle = self.tail_lock.write().unwrap();
                state_handle.simulate_tick(sim_info, FRAME_DURATION_MILLIS);
                self.data_store.set_tail_frame(tail_frame_to_sim as i32);
//                println!("TailSim {}", state_handle.get_simmed_frame_index());
                return vec![];
            }
            Err(problems) =>{
                return problems;
            }
        }
    }


    fn start_thread(mut self, outwards_messages: Sender<QuerySimData>, mut new_tails_sink: Sender<GameState>, new_hashes: Sender<FramedHash>){
        thread::spawn(move ||
        {
            let mut first_frame_to_sim = self.tail_lock.read().unwrap().get_simmed_frame_index() + 1;
            println!("Logic next frame to sim: {}", first_frame_to_sim);
            let mut generator = self.known_frame_info.start_frame_stream_from_any(first_frame_to_sim);
            loop{
                let dt_get_frame = DT::start_fmt(format!("to get a frame"));
                let frame_to_sim = generator.recv().unwrap();
                dt_get_frame.stop();
                let dt = DT::start_fmt(format!("to sim frame {}", frame_to_sim));
                loop{
                    let problems = self.try_sim_tail_frame(frame_to_sim);
                    if problems.is_empty(){
                        break;
                    }else{
                        for problem in &problems{
                            println!("Logic missing info so asking: {:?}", problem);
                            outwards_messages.send( problem.clone() ).unwrap();
                        }
                        thread::sleep(Duration::from_millis(1000)); // modival Resend period.
                    }
                }
                let new_tail = self.tail_lock.read().unwrap().clone();
                new_hashes.send(FramedHash::new(frame_to_sim, new_tail.get_hash())).unwrap();
                new_tails_sink.send(new_tail).unwrap(); // Send new head regardless of success.

                dt.stop();
            }
        });
    }

    pub fn start_logic_tail(mut self) -> LogicSimTailerEx {
        let (from_logic_sink, from_logic_rec) = unbounded();
        let (new_tails_sink, new_tails_rec) = unbounded();
        let (new_hashes_sink, new_hashes_rec) = unbounded();


        let tail_lock = self.tail_lock.clone();

        self.start_thread(from_logic_sink, new_tails_sink, new_hashes_sink);

        LogicSimTailerEx {
            from_logic_rec,
            tail_lock,
            new_tail_states_rec: Some(new_tails_rec),
            new_tail_hashes: Some(new_hashes_rec)
        }
    }
}