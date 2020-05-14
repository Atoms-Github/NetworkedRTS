use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::network::networking_message_types::*;
use crate::network::networking_structs::*;
use crate::players::inputs::*;

use crate::game::logic::logic_segment::*;
use crate::game::logic::logic_head_sim_segment::*;
use crate::game::synced_data_stream::*;
use crate::game::timekeeping::*;

//trait AbleToStartCollectionThread {
//    fn init_input_collector_thread(self, known_frame: KnownFrameInfo, first_frame_to_send: FrameIndex) -> Receiver<(FrameIndex, InputState)>;
//}
//impl AbleToStartCollectionThread for Receiver<InputChange>{
//    fn init_input_collector_thread(self, known_frame: KnownFrameInfo, first_frame_to_send: FrameIndex) -> Receiver<(FrameIndex, InputState)> {
//        let mut frame_generator = known_frame.start_frame_stream_from_any(first_frame_to_send - HEAD_AHEAD_FRAME_COUNT);
//        let (merged_sink, merged_rec) = channel();
//        thread::spawn(move ||{
//            let mut current_input_state = InputState::new();
//            loop{
//                let tail_frame_index = frame_generator.recv().unwrap(); // Wait for new frame.
//
//                let mut change = self.try_recv();
//                while change.is_ok(){ // Keep fishing.
//                    change.unwrap().apply_to_state(&mut current_input_state);
//                    change = self.try_recv();
//                }
//                merged_sink.send((tail_frame_index + HEAD_AHEAD_FRAME_COUNT , current_input_state.clone())).unwrap();
//            }
//        });
//        return merged_rec;
//    }
//}


pub struct InputHandlerEx {
//    inputs_stream_state: Receiver<InputChange>,
//    to_logic: Sender<LogicInwardsMessage>,
//    known_frame: KnownFrameInfo,
//    player_id: PlayerID
}
impl InputHandlerEx {

}
enum InputHandlerMsg{
    NewFrame(FrameIndex),
    InputsUpdate(InputChange)
}
pub struct InputHandlerIn {
    known_frame: KnownFrameInfo,
    player_id: PlayerID,
//    first_frame_to_send: FrameIndex,
    to_logic: Sender<LogicInwardsMessage>,
    to_net: Sender<NetMessageType>,
}
impl InputHandlerIn {
    // This segment's job is to get the user's inputs and send them to the logic data storage and the network.
    // We want the network sender to gather them and send at end of frame.
    // We want local logic to be firing off ASAP.
    //
    // We can't duplicate the input receiver and have two separate gathering methods as it's super important that the frame count output of each of them is in sync.
    // So we're going to have one thread with a receiver which either gets a message to send away straight away, or a time notification meaning send net with current.

    pub fn new(known_frame: KnownFrameInfo, player_id: PlayerID, /*first_frame_to_send: FrameIndex,*/ to_logic: Sender<LogicInwardsMessage>, to_net: Sender<NetMessageType>,) -> InputHandlerIn {
        return InputHandlerIn {
            known_frame,
            player_id,
//            first_frame_to_send,
            to_logic,
            to_net
        }
    }


    fn gen_time_msgs_th(&self, out_msgs: Sender<InputHandlerMsg>){

        let tail_frame_rec = self.known_frame.start_frame_stream_from_known();
        thread::spawn(move ||{
            loop{
                let head_frame = tail_frame_rec.recv().unwrap() + HEAD_AHEAD_FRAME_COUNT;
                out_msgs.send(InputHandlerMsg::NewFrame(head_frame));
            }
        });
    }
    fn forward_input_changes_th(&self, inc_states: Receiver<InputChange>, out_msgs: Sender<InputHandlerMsg>){
        thread::spawn(move ||{
            loop{
                let next_change = inc_states.recv().unwrap();
                out_msgs.send(InputHandlerMsg::InputsUpdate(next_change)).unwrap();
            }
        });
    }
    fn generate_msg_stream(&self, input_changes: Receiver<InputChange>) -> Receiver<InputHandlerMsg>{
        let (mut handler_msg_sink, mut handler_msg_rec) = channel();
        self.gen_time_msgs_th(handler_msg_sink.clone());
        self.forward_input_changes_th(input_changes, handler_msg_sink.clone());
        return handler_msg_rec;
    }


    pub fn start_dist(self, inputs_stream: Receiver<InputChange>) -> InputHandlerEx{
        thread::spawn(move ||{

            let handler_msg_rec = self.generate_msg_stream(inputs_stream);
            let mut curret_input = InputState::new();
            let mut inputs_arriving_for_frame = self.known_frame.get_intended_current_frame() + HEAD_AHEAD_FRAME_COUNT;
            loop{
                let next_message = handler_msg_rec.recv().unwrap();
                match &next_message{
                    InputHandlerMsg::InputsUpdate(input_change) => {
                        input_change.apply_to_state(&mut curret_input);
                    }
                    _ => {}
                }
                let logic_message = LogicInwardsMessage::SyncerInputsUpdate(SyncerData{
                    data: vec![curret_input.clone()],
                    start_frame: inputs_arriving_for_frame,
                    owning_player: self.player_id,
                });
                match next_message{
                    InputHandlerMsg::NewFrame(next_frame_index) => {
                        self.to_net.send(NetMessageType::GameUpdate(logic_message)).unwrap();
                        inputs_arriving_for_frame = next_frame_index;
                    }
                    InputHandlerMsg::InputsUpdate(input_change) => {
                        self.to_logic.send(logic_message.clone()).unwrap();
                    }
                }
            }
        });

        return InputHandlerEx{
        }
    }
}


