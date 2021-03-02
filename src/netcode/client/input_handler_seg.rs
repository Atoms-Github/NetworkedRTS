use crossbeam_channel::*;
use std::thread;

use crate::netcode::client::logic_sim_header_seg::*;
use crate::netcode::common::logic::logic_sim_tailer_seg::*;
use crate::netcode::common::sim_data::input_state::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::common::sim_data::sim_data_storage::*;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use crate::netcode::common::network::external_msg::ExternalMsg;
use crate::netcode::common::sim_data::superstore_seg::SuperstoreData;

pub struct InputHandler {
    player_id: PlayerID,
    inputs_stream: Receiver<InputChange>,
    curret_input: InputState,
    to_net: Sender<(ExternalMsg,bool)>
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
    fn apply_input_changes(&mut self){
        loop{
            let mut next_input = self.inputs_stream.try_recv();
            match next_input{
                Ok(input_change) => {
                    input_change.apply_to_state(&mut self.curret_input);
                }
                Err(e) => {
                    return;
                }
            }
        }
    }
    pub fn update(&mut self, data_store: &mut SimDataStorage, inputs_arriving_for_frame: FrameIndex){
        self.apply_input_changes();
        data_store.write_input_data_single(self.player_id, self.curret_input.clone(), inputs_arriving_for_frame);

        let data = SuperstoreData{
            data: vec![self.curret_input.clone()],
            frame_offset: inputs_arriving_for_frame
        };
        self.to_net.send((ExternalMsg::GameUpdate(SimDataPackage::PlayerInputs(data, self.player_id)), false)).unwrap();
    }
}

