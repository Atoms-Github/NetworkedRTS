use std::thread;
use std::sync::{Arc, RwLock};
use crossbeam_channel::*;

use crate::netcode::*;
use crate::netcode::common::sim_data::confirmed_data::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use crate::netcode::common::sim_data::net_game_state::{NetPlayerProperty, NetGameState};
use std::sync::mpsc::channel;


pub const HEAD_AHEAD_FRAME_COUNT: usize = 20;

pub struct HeadSimPacket{
    pub game_state: NetGameState,
    pub sim_data: Vec<InfoForSim>,
}

pub struct LogicSimHeaderIn {
    head_sim_packets_rec: Receiver<HeadSimPacket>,
}
pub struct LogicSimHeaderEx {
    pub uncalculated_heads: Sender<HeadSimPacket>,
    pub calculated_heads: Option<Receiver<NetGameState>>
}
impl LogicSimHeaderEx{
    // TODO: Start the graphics somewhere in here.
    pub fn start(new_heads: Receiver<HeadSimPacket>) -> Self {
        let (head_sim_packets_tx, head_sim_packets_rec) = unbounded();
        LogicSimHeaderIn {
            head_sim_packets_rec,
        }.start(head_sim_packets_tx);
        GraphicalEx::new(seg_logic_header.calculated_heads.take().unwrap(), welcome_info.assigned_player_id);
    }
    pub fn get_head_sim_data(&self, data_store: &ConfirmedData, first_frame_to_include : FrameIndex) -> Vec<InfoForSim>{
        let mut sim_infos = vec![];
        for frame_index in (first_frame_to_include)..(first_frame_to_include + HEAD_AHEAD_FRAME_COUNT){
            let mut sim_info = InfoForSim{
                inputs_map: Default::default(),
                server_events: data_store.get_server_events_or_empty(frame_index)
            };
            for player_id in data_store.get_player_list(){
                let this_players_input = match data_store.get_input(frame_index, player_id){
                    Some(input_state) => {
                        input_state.clone()
                    }
                    None => {
                        // Get my previous frame's input.
                        let maybe_info_for_sim: Option<&InfoForSim> = sim_infos.last();
                        if let Some(info_for_sim) = maybe_info_for_sim{
                            info_for_sim.inputs_map.get(&player_id).clone().unwrap().clone()
                        }else{
                            InputState::new()
                        }

                    }
                };
                sim_info.inputs_map.insert(player_id, this_players_input);
            }
            sim_infos.push(sim_info);
        }
        return sim_infos;
    }
    pub fn send_head_state(&mut self, gamestate: NetGameState, data_store: &ConfirmedData){
        let sim_data = self.get_head_sim_data(data_store, gamestate.get_simmed_frame_index() + 1);
        let head_packet = HeadSimPacket{
            game_state: gamestate,
            sim_data
        };
        self.uncalculated_heads.send(head_packet).unwrap();
    }
}

impl LogicSimHeaderIn {
    pub fn start(mut self, new_head_states: Sender<HeadSimPacket>) -> LogicSimHeaderEx{
        let (mut head_sink, mut head_rec) = unbounded();
        thread::spawn(move ||{
            loop{
                // Skip any we're lagging by - we're only interested in latest.
                let head_sim_packet = crate::utils::pull_latest(&mut self.head_sim_packets_rec);
                log::trace!("Head got frame {}", head_sim_packet.game_state.get_simmed_frame_index());

                let new_head = calculate_new_head(head_sim_packet);
                head_sink.send(new_head).unwrap();

            }
        });

        LogicSimHeaderEx{
            uncalculated_heads: new_head_states,
            calculated_heads : Some(head_rec)
        }
    }
}
fn calculate_new_head(mut sim_packet: HeadSimPacket) -> NetGameState {
    for sim_info in sim_packet.sim_data{
        let metadata = SimMetadata{
            delta: FRAME_DURATION_MILLIS,
            quality: SimQuality::HEAD,
            frame_index: sim_packet.game_state.get_simmed_frame_index() + 1,
        };
        sim_packet.game_state.simulate_tick(sim_info, &metadata );
    }
    return sim_packet.game_state;
}

