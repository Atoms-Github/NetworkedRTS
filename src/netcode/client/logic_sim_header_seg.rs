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
    head_sim_packets_rec: Receiver<HeadSimPacket>,
    data_store: SimDataStorage,
}
pub struct LogicSimHeaderEx {
    pub new_head_states: Sender<HeadSimPacket>,
}
impl LogicSimHeaderEx{
    pub fn start(known_frame_info: KnownFrameInfo, head_sim_packets_rec: Receiver<HeadSimPacket>, data_store: SimDataStorage) -> Self {
        LogicSimHeaderIn {
            known_frame_info,
            data_store,
            head_sim_packets_rec
        }.start()
    }
    pub fn get_head_sim_data(&self, data_store: &SimDataStorage, first_frame_to_include : FrameIndex) -> Vec<InfoForSim>{
        let mut sim_infos = vec![];
        for frame_index in (first_frame_to_include)..(first_frame_to_include + HEAD_AHEAD_FRAME_COUNT){
            let mut sim_info = InfoForSim{
                inputs_map: Default::default(),
                server_events: data_store.get_server_events_or_empty(frame_index)
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
                let head_sim_packet = self.head_sim_packets_rec.recv().unwrap();
                log::trace!("Head got frame {}", head_sim_packet.game_state.get_simmed_frame_index());

                let new_head = self.calculate_new_head(head_sim_packet);
                head_sink.send(new_head).unwrap();

            }
        });

        LogicSimHeaderEx{
            new_head_states: head_rec
        }
    }
    fn calculate_new_head(&mut self, mut sim_packet: HeadSimPacket) -> NetGameState {
        for sim_info in sim_packet.sim_data{
            sim_packet.game_state.simulate_tick(sim_info, FRAME_DURATION_MILLIS);
        }
        return sim_packet.game_state;
    }
}

