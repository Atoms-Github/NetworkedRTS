use crossbeam_channel::*;
use std::thread;

use crate::netcode::client::logic_sim_header_seg::*;
use crate::netcode::common::logic::logic_sim_tailer_seg::*;
use crate::netcode::common::sim_data::framed_vec::*;
use crate::netcode::common::sim_data::input_state::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::common::sim_data::sim_data_storage::*;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;

pub struct InputHandlerEx {
//    inputs_stream_state: Receiver<InputChange>,
//    to_logic: Sender<LogicInwardsMessage>,
//    known_frame: KnownFrameInfo,
//    player_id: PlayerID
}
impl InputHandlerEx {
    pub fn start(known_frame: KnownFrameInfo, player_id: PlayerID, first_frame_to_send: FrameIndex,
               inputs_stream: Receiver<InputChange>, sim_data_storage: SimDataStorageEx,) -> InputHandlerEx {
        InputHandlerIn {
            known_frame,
            player_id,
            sim_data_storage,
            inputs_stream,
            curret_input: InputState::new(),
            next_frame_to_send: first_frame_to_send,
            inputs_arriving_for_frame: std::usize::MAX
        }.start()
    }
}
#[derive()]
pub struct InputHandlerIn {
    known_frame: KnownFrameInfo,
    player_id: PlayerID,
    sim_data_storage: SimDataStorageEx,
    inputs_stream: Receiver<InputChange>,
    curret_input: InputState,
    next_frame_to_send: FrameIndex,
    inputs_arriving_for_frame: FrameIndex,
}
impl InputHandlerIn {
    // This segment's job is to get the user's inputs and just send them on to the data storage.

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
    // dcwct Also no need for delaying initialization of this system - it needs to accurately know itself which frames to send anyway.
    pub fn start(mut self) -> InputHandlerEx{
        thread::spawn(move ||{
            let frame_gen = self.known_frame.start_frame_stream_from_now();
            loop{
                let tail_frame = frame_gen.recv().unwrap();
                let head_frame = tail_frame + HEAD_AHEAD_FRAME_COUNT;
                if head_frame == self.next_frame_to_send{
                    // 1. Apply any inputs.
                    self.apply_input_changes();
                    // 2. Send it off.
                    log::trace!("Sending local input for frame: {}", head_frame);
                    self.sim_data_storage.write_data_single(self.player_id, self.curret_input.clone(), head_frame);
                    // 3. Increment next_frame_to_send.
                    self.next_frame_to_send += 1;
                }
            }

        });
        InputHandlerEx{}
    }
}


