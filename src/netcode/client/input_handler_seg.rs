use crossbeam_channel::*;
use std::thread;

use crate::netcode::client::logic_sim_header_seg::*;
use crate::netcode::common::logic::logic_sim_tailer_seg::*;
use crate::netcode::common::sim_data::input_state::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::common::sim_data::confirmed_data::*;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use crate::netcode::common::network::external_msg::ExternalMsg;
use crate::netcode::common::sim_data::superstore_seg::SuperstoreData;

pub struct InputHandler {
    player_id: PlayerID,
    inputs_stream: Receiver<InputChange>,

    to_net: Sender<(ExternalMsg,bool)> // TODO Don't need any sort of threads. Don't do new thread for this.
}
impl InputHandler {
    pub fn new(player_id: PlayerID, inputs_stream: Receiver<InputChange>, to_net: Sender<(ExternalMsg,bool)>) -> Self{
        Self{
            player_id,
            inputs_stream,
            curret_input: InputState::new(),
            to_net
        }
    }

    pub fn update(&mut self, data_store: &mut ConfirmedData, inputs_arriving_for_frame: FrameIndex){
        
        }

    }
}

