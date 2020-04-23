use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::network::networking_message_types::*;
use crate::network::networking_structs::*;
use crate::players::inputs::*;

use crate::game::logic::logic_segment::*;
use crate::game::synced_data_stream::*;
use crate::game::timekeeping::*;

trait AbleToStartCollectionThread {
    fn init_input_collector_thread(self, known_frame: KnownFrameInfo) -> Receiver<(FrameIndex, InputState)>;
}
impl AbleToStartCollectionThread for Receiver<InputChange>{
    fn init_input_collector_thread(self, known_frame: KnownFrameInfo) -> Receiver<(FrameIndex, InputState)> {
        let mut frame_generator = known_frame.start_frame_stream_from_any(known_frame.get_intended_current_frame());
        let (merged_sink, merged_rec) = channel();
        thread::spawn(move ||{
            loop{
                let frame_index = frame_generator.recv().unwrap(); // Wait for new frame.
                let mut input_state = InputState::new();

                let mut change = self.try_recv();
                while change.is_ok(){ // Keep fishing.
                    change.unwrap().apply_to_state(&mut input_state);
                    change = self.try_recv();
                }
                merged_sink.send((frame_index, input_state)).unwrap();
            }
        });
        return merged_rec;
    }
}

pub struct InputHandlerEx {
//    inputs_stream_state: Receiver<InputChange>,
//    to_logic: Sender<LogicInwardsMessage>,
//    known_frame: KnownFrameInfo,
//    player_id: PlayerID
}
impl InputHandlerEx {

}
pub struct InputHandlerIn {
    inputs_stream_changes: Receiver<InputChange>,
    known_frame: KnownFrameInfo,
    player_id: PlayerID,
}
impl InputHandlerIn {
    pub fn new(inputs_stream_state: Receiver<InputChange>, known_frame: KnownFrameInfo, player_id: PlayerID) -> InputHandlerIn {
        return InputHandlerIn {
            inputs_stream_changes: inputs_stream_state,
            known_frame,
            player_id
        }
    }


    pub fn start_dist(self, to_logic: Sender<LogicInwardsMessage>, to_net: Sender<NetMessageType>) -> InputHandlerEx{
        let grouped_inputs = self.inputs_stream_changes.init_input_collector_thread(self.known_frame.clone());
        let my_player_id = self.player_id;
        thread::spawn(move ||{
            loop{
                let (frame_index, state) = grouped_inputs.recv().unwrap();
//                let now_frame_index = my_known_frame.get_intended_current_frame(); // Super important this doesn't change between local and sent so we get here.

                let logic_message = LogicInwardsMessage::SyncerInputsUpdate(SyncerData{
                    data: vec![state],
                    start_frame: frame_index,
                    owning_player: my_player_id as i32
                });
                println!("Sent y'all frame number {}", frame_index);
                to_logic.send(logic_message.clone()).unwrap();
                to_net.send(NetMessageType::GameUpdate(logic_message)).unwrap();
            }
        });

        return InputHandlerEx{
        }
    }
}


