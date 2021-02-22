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

pub struct HeadSimPacket{
    pub game_state: NetGameState,
    pub sim_data: Vec<InfoForSim>,
}

pub struct LogicSimHeaderIn {
    known_frame_info: KnownFrameInfo,
    tail_rec: Receiver<NetGameState>,
    data_store: SimDataStorage,
}
pub struct LogicSimHeaderEx {
    pub head_rec: Option<Receiver<NetGameState>>,
    pub new_head_states: Sender<HeadSimPacket>, // breaking: Implement
}
impl LogicSimHeaderEx{
    pub fn start(known_frame_info: KnownFrameInfo, tail_rec: Receiver<NetGameState>, data_store: SimDataStorage) -> Self {
        LogicSimHeaderIn {
            known_frame_info,
            data_store,
            tail_rec
        }.start()
    }
    pub fn get_head_sim_data(&self, data_store: &SimDataStorage, first_frame_to_include : FrameIndex) -> Vec<InfoForSim>{
        let mut sim_infos = vec![];
        for frame_index in (first_frame_to_include)..(first_frame_to_include + HEAD_AHEAD_FRAME_COUNT){
            let mut sim_info = InfoForSim{
                inputs_map: Default::default(),
                server_events: data_store.get_server_events_or_empty(frame_index);
            };
            for player_id in data_store.get_player_list(){
                if let Some(input_state) = data_store.get_input(frame_index, player_id){
                    sim_info.inputs_map.insert(player_id, input_state);
                }
            }

            sim_infos.push(sim_info);
        }
        return sim_infos;
    }
    pub fn send_head_state(&mut self, gamestate: NetGameState, data_store: &SimDataStorage){




        let head_packet = HeadSimPacket{
            game_state: gamestate,
            sim_data: self.get_head_sim_data(data_store, gamestate.get_simmed_frame_index() + 1)
        };
        self.new_head_states.send(head_packet).unwrap();
    }
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

