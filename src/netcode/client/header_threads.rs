use std::thread;
use std::sync::{Arc, RwLock};
use crossbeam_channel::*;

use crate::netcode::*;
use crate::netcode::common::timekeeping::*;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use std::sync::mpsc::channel;
use crate::netcode::client::graphical_seg::GraphicalEx;
use crate::netcode::common::net_game_state::NetGameState;


pub const HEAD_AHEAD_FRAME_COUNT: usize = 20;

pub struct HeadSimPacket{
    pub game_state: NetGameState,
    pub sim_data: Vec<InfoForSim>,
}

pub struct HeaderThread {
    head_sim_packets_rec: Receiver<HeadSimPacket>,
    output_states: Sender<NetGameState>,
}
impl HeaderThread {
    pub fn start(new_heads: Receiver<HeadSimPacket>, my_player_id: PlayerID) {
        let (tx_output_states, rx_output_states) = unbounded();
        thread::spawn(move ||{
            HeaderThread {
                head_sim_packets_rec: new_heads,
                output_states: tx_output_states,
            }.start_loop();
        });
        GraphicalEx::new(rx_output_states, my_player_id);
    }
    pub fn start_loop(mut self) -> !{
        loop{
            // Skip any we're lagging by - we're only interested in latest. TODish: We could partially sim one, and move to next.
            // TBH, you may well prefer 30fps and no ping, than 60fps. For consideration ...
            let head_sim_packet = crate::utils::pull_latest(&mut self.head_sim_packets_rec);
            log::trace!("Head got frame {}", head_sim_packet.game_state.get_simmed_frame_index());

            let new_head = calculate_new_head(head_sim_packet);
            self.output_states.send(new_head).unwrap();

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

        sim_packet.game_state.simulate(sim_info, &metadata );
    }
    return sim_packet.game_state;
}

