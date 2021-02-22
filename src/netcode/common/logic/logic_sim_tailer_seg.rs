use std::thread;
use std::sync::{Arc, RwLock};
use crossbeam_channel::*;

use serde::{Deserialize, Serialize};

use crate::netcode::*;
use crate::netcode::common::sim_data::input_state::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use crate::netcode::common::sim_data::superstore_seg::*;
use crate::netcode::common::sim_data::sim_data_storage::*;
use std::time::{SystemTime, Duration};

use crate::netcode::common::logic::hash_seg::*;
use std::hash::Hash;
use crate::netcode::common::sim_data::net_game_state::{NetPlayerProperty, NetGameState};


pub struct LogicSimTailer {
    pub game_state: NetGameState,
    pub known_frame: KnownFrameInfo,
}
impl LogicSimTailer{
    pub fn new(game_state: NetGameState, known_frame: KnownFrameInfo) -> Self{
        Self{
            game_state,
            known_frame
        }
    }
    // breaking: Need to be able to sim two in one call.
    fn simulate_frame(&mut self, data_store: &SimDataStorage) -> Result<InfoForSim, Vec<SimDataQuery>>{
        let frame_to_sim = self.game_state.get_simmed_frame_index() + 1;

        let mut player_inputs: HashMap<PlayerID, InputState> = Default::default();
        let mut problems = vec![];

        for (player_id, player_property) in self.game_state.players{
            if let Some(input_state) = data_store.get_input(frame_to_sim, player_id){
                    player_inputs.insert(*player_id, state);
            }else{
                problems.push(SimDataQuery {
                    query_type: SimDataOwner::Player(),
                    frame_offset: frame_index,
                    player_id: waiting_id
                });
            }
        }
        // breaking get server events too.

        if !problems.is_empty(){
            return Err(problems);

        }
        return Ok(InfoForSim{
            inputs_map: player_inputs
        });
    }
    pub fn catchup_simulation(&mut self, data_store: &SimDataStorage, sim_frame_up_to_and_including: FrameIndex) -> Result<(), Vec<SimDataQuery>>{
        let frame_to_sim = self.game_state.get_simmed_frame_index() + 1;
        // breaking catchup a bit using self.known_frame. Also implement limit of 3 sims.
    }
}


pub struct LogicSimTailerEx {
    pub from_logic_rec: Receiver<SimDataQuery>,
    pub tail_lock: ArcRw<NetGameState>,
    pub new_tail_states_rec: Option<Receiver<NetGameState>>,
    pub new_tail_hashes: Option<Receiver<FramedHash>>,
}
impl LogicSimTailerEx {
    pub fn start(known_frame_info: KnownFrameInfo, state_tail: NetGameState, data_store: SimDataStorage) -> Self {
        LogicSimTailerIn {
            known_frame_info,
            tail_lock: Arc::new(RwLock::new(state_tail)),
            data_store
        }.start_logic_tail()
    }
}
pub struct LogicSimTailerIn {
    known_frame_info: KnownFrameInfo,
    tail_lock: ArcRw<NetGameState>,
    data_store: SimDataStorage
    // Logic layer shouldn't know it's player ID.
}

impl LogicSimTailerIn {
    fn try_sim_tail_frame(&mut self, tail_frame_to_sim: FrameIndex) -> Vec<SimDataQuery>{

        let mut state_handle = self.tail_lock.write().unwrap();

        let who_we_wait_for = state_handle.get_who_we_wait_for();
        // TODO1: note for next time: Just change this here? Is it ok that the data_store is a thing we're trying to keep in sync with no other interesting state,
        // TODO1: and to keep state in NetGameState?. I think so. So you can pass NetGameState into clone for tail or something similar.
        let sim_query_result = self.data_store.clone_info_for_tail(tail_frame_to_sim, who_we_wait_for);
        match sim_query_result{
            Ok(sim_info) => {


                state_handle.simulate_tick(sim_info, FRAME_DURATION_MILLIS);
                self.data_store.set_tail_frame(tail_frame_to_sim as i32);
                log::trace!("TailSim {}", state_handle.get_simmed_frame_index());
                return vec![];
            }
            Err(problems) =>{
                return problems;
            }
        }
    }


    fn start_thread(mut self, outwards_messages: Sender<SimDataQuery>, mut new_tails_sink: Sender<NetGameState>, new_hashes: Sender<FramedHash>){
        thread::spawn(move ||
        {
            let mut first_frame_to_sim = self.tail_lock.read().unwrap().get_simmed_frame_index() + 1;
            log::trace!("Logic next frame to sim: {}", first_frame_to_sim);
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
                            log::warn!("Logic missing info so asking: {:?}", problem);
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

    fn start_logic_tail(mut self) -> LogicSimTailerEx {
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