use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::game::graphical_segment::GraphicalSegment;
use crate::network::networking_message_types::*;
use crate::network::networking_segment::*;
use crate::network::networking_structs::*;
use crate::players::inputs::*;
use std::panic;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use crate::game::logic::logic_segment::*;
use crate::game::logic::logic_head_sim_segment::*;
use crate::game::synced_data_stream::*;
use crate::game::timekeeping::*;
use crate::game::logic::data_storage_manager::*;
use crate::game::logic::logic_data_storage::*;


pub struct InputHandlerEx {
//    inputs_stream_state: Receiver<InputChange>,
//    to_logic: Sender<LogicInwardsMessage>,
//    known_frame: KnownFrameInfo,
//    player_id: PlayerID
}
impl InputHandlerEx {

}
pub struct InputHandlerIn {
    inputs_stream_state: Receiver<InputChange>,
    known_frame: KnownFrameInfo,
    player_id: PlayerID,
}
impl InputHandlerIn {
    pub fn new(inputs_stream_state: Receiver<InputChange>, known_frame: KnownFrameInfo, player_id: PlayerID) -> InputHandlerIn {
        return InputHandlerIn {
            inputs_stream_state,
            known_frame,
            player_id
        }
    }
    fn init_input_collector_thread(&self, inputs_rec: Receiver<InputChange>) -> Receiver<InputState>{ // TODO3: Things can be improved by not waiting for the entire frame to finish before sending the entire input frame to local logic. Could be as it comes.
        let mut frame_generator = self.known_frame.start_frame_stream_from_known();
        let (merged_sink, merged_rec) = channel();

        thread::spawn(move ||{
            loop{
                let frame_index = frame_generator.recv().unwrap(); // Wait for new frame.
                let mut input_state = InputState::new();

                let mut change = inputs_rec.try_recv();
                while change.is_ok(){ // Keep fishing.
                    change.unwrap().apply_to_state(&mut input_state);
                    change = inputs_rec.try_recv();
                }
                merged_sink.send(input_state).unwrap();
            }
        });
        return merged_rec;
    }

    pub fn start_dist(self, to_logic: Sender<LogicInwardsMessage>, to_net: Sender<NetMessageType>) -> InputHandlerEx{
        let grouped_inputs = self.init_input_collector_thread(self.inputs_stream_state);
        thread::spawn(move ||{
            loop{
                let state = self.inputs_stream_state.recv().unwrap();
                let now_frame_index = self.known_frame.get_intended_current_frame(); // Super important this doesn't change between local and sent so we get here.

                let logic_message = LogicInwardsMessage::SyncerInputsUpdate(SyncerData{
                    data: vec![],
                    start_frame: now_frame_index,
                    owning_player: self.player_id as i32
                });
                to_logic.send(logic_message.clone()).unwrap();
                to_net.send(NetMessageType::GameUpdate(logic_message)).unwrap();
            }
        });

        return InputHandlerEx{
        }
    }
}


