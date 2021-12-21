use std::thread;
use std::sync::{Arc, RwLock};
use crossbeam_channel::*;

use crate::*;
use crate::common::timekeeping::*;
use crate::pub_types::*;
use std::sync::mpsc::channel;
use crate::common::net_game_state::{NetGameState, GameState};
use crate::client::graphical_seg::GraphicalIn;
use crate::common::input_state::InputChange;


pub const HEAD_AHEAD_FRAME_COUNT: usize = 20;

pub struct HeadSimPacket<T : 'static + GameState>{
    pub game_state: NetGameState<T>,
    pub sim_data: Vec<InfoForSim>,
}

pub struct HeaderThread<T : 'static + GameState> {
    head_sim_packets_rec: Receiver<HeadSimPacket<T>>,
    output_states: Sender<NetGameState<T>>,
}
impl<T : 'static + GameState> HeaderThread<T> {
    pub fn start(inputs: Sender<InputChange>, new_heads: Receiver<HeadSimPacket<T>>, my_player_id: PlayerID) {
        let (tx_output_states, rx_output_states) = unbounded();
        thread::spawn(move ||{
            HeaderThread {
                head_sim_packets_rec: new_heads,
                output_states: tx_output_states,
            }.start_loop();
        });
        GraphicalIn::start(inputs, rx_output_states, my_player_id);
    }
    pub fn start_loop(mut self) -> !{
        loop{
            // Skip any we're lagging by - we're only interested in latest. TODish: We could partially sim one, and move to next.
            // TBH, you may well prefer 30fps and no ping, than 60fps. For consideration ...
            let head_sim_packet = crate::utils::pull_latest(&mut self.head_sim_packets_rec);
            log::trace!("Head got frame {}", head_sim_packet.game_state.get_simmed_frame_index());

            let new_head = self.calculate_new_head(head_sim_packet);
            self.output_states.send(new_head).unwrap();

        }
    }
    fn calculate_new_head(&self, mut sim_packet: HeadSimPacket<T>) -> NetGameState<T> {
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
}


