use std::sync::mpsc::{Receiver};
use std::thread;

use crate::client::logic_sim_header_seg::*;
use crate::common::logic::logic_sim_tailer_seg::*;
use crate::common::sim_data::framed_vec::*;
use crate::common::sim_data::input_state::*;
use crate::common::time::timekeeping::*;
use crate::common::sim_data::sim_data_storage::*;
use crate::common::types::*;
use std::sync::{Arc, RwLock};


pub struct InputHandlerEx {
//    inputs_stream_state: Receiver<InputChange>,
//    to_logic: Sender<LogicInwardsMessage>,
//    known_frame: KnownFrameInfo,
//    player_id: PlayerID
}
impl InputHandlerEx {

}
#[derive()]
pub struct InputHandlerIn {
    known_frame: KnownFrameInfo,
    player_id: PlayerID,
    sim_data_storage: SimDataStorageEx,
    inputs_stream: Receiver<InputChange>,
    curret_input: InputState,
    inputs_arriving_for_frame: FrameIndex,
}
impl InputHandlerIn {
    // This segment's job is to get the user's inputs and just send them on to the data storage.
    pub fn new(known_frame: KnownFrameInfo, player_id: PlayerID, first_frame_to_send: FrameIndex,
               inputs_stream: Receiver<InputChange>, sim_data_storage: SimDataStorageEx,) -> InputHandlerIn {
        InputHandlerIn {
            known_frame,
            player_id,
            sim_data_storage,
            inputs_stream,
            curret_input: InputState::new(),
            inputs_arriving_for_frame: std::usize::MAX
        }
    }
    // You should be able to send anything you want to

    pub fn start_dist(mut self) -> InputHandlerEx{
        thread::spawn(move ||{
            let next_input = self.inputs_stream.recv().unwrap();
            next_input.apply_to_state(&mut self.curret_input);
            // TODO2: What if there are no inputs from player?
            let mut inputs_arriving_for_frame = self.known_frame.get_intended_current_frame() + HEAD_AHEAD_FRAME_COUNT;

            self.sim_data_storage.write_data_single(self.player_id, self.curret_input.clone(), inputs_arriving_for_frame);
        });

        InputHandlerEx{}
    }
}

