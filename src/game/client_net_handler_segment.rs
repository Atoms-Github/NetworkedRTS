//use std::net::SocketAddr;
//use std::str::FromStr;
//use std::sync::mpsc::{channel, Receiver, Sender};
//use std::thread;
//
//use crate::game::graphical_segment::GraphicalSegment;
//use crate::network::networking_message_types::*;
//use crate::network::networking_segment::*;
//use crate::network::networking_structs::*;
//use crate::players::inputs::*;
//use std::panic;
//use std::sync::{Arc, Mutex, RwLock};
//use std::time::Duration;
//
//use crate::game::logic::logic_segment::*;
//use crate::game::logic::logic_head_sim_segment::*;
//use crate::game::synced_data_stream::*;
//use crate::game::timekeeping::*;
//use crate::game::logic::data_storage_manager::*;
//use crate::game::logic::logic_data_storage::*;
//
//
//struct ClientNetHandlerSegEx {
//    inputs_stream_state: Receiver<InputState>,
//    outgoing_network: Sender<NetMessageType>,
//    to_logic: Sender<LogicInwardsMessage>,
//    known_frame: KnownFrameInfo,
//    player_id: PlayerID
//}
//struct ClientNetHandlerSegIn {
//    inputs_stream_state: Receiver<InputState>,
//    outgoing_network: Sender<NetMessageType>,
//    to_logic: Sender<LogicInwardsMessage>,
//    known_frame: KnownFrameInfo,
//    player_id: PlayerID
//}
//impl ClientNetHandlerSegIn {
//    pub fn new() -> ClientNetHandlerSegIn {
//        return ClientNetHandlerSegIn {
//            inputs_stream_state: changes_rec,
//            outgoing_network,
//            to_logic,
//            known_frame: welcome_info.known_frame_info.clone(),
//            player_id: welcome_info.assigned_player_id
//        }
//    }
//}
//impl ClientNetHandlerSegEx {
//    fn start(self){
//        thread::spawn(move ||{
//            loop{
//                let state = self.inputs_stream_state.recv().unwrap();
//                let now_frame_index = self.known_frame.get_intended_current_frame(); // Super important this doesn't change between local and sent so we get here.
//
//                let logic_message = LogicInwardsMessage::SyncerInputsUpdate(SyncerData{
//                    data: vec![],
//                    start_frame: now_frame_index,
//                    owning_player: self.player_id as i32
//                });
//                self.to_logic.send(logic_message.clone()).unwrap();
//                self.outgoing_network.send(NetMessageType::GameUpdate(logic_message)).unwrap();
//            }
//        });
//    }
//    pub fn init_input_collector_thread(inputs_rec: Receiver<InputChange>, known_frame: KnownFrameInfo) -> Receiver<InputState>{
//        let mut frame_generator = known_frame.start_frame_stream_from_known();
//        let (merged_sink, merged_rec) = channel();
//
//        thread::spawn(move ||{
//            loop{
//                let frame_index = frame_generator.recv().unwrap(); // Wait for new frame.
//                let mut input_state = InputState::new();
//
//                let mut change = inputs_rec.try_recv();
//                while change.is_ok(){ // Keep fishing.
//                    change.unwrap().apply_to_state(&mut input_state);
//                    change = inputs_rec.try_recv();
//                }
//                merged_sink.send(input_state).unwrap();
//            }
//        });
//        return merged_rec;
//    }
//}
//
//fn init_logic_output_responder(logic_output: Receiver<LogicOutwardsMessage>, network_sink: Sender<NetMessageType>, input_distributor: InputDistributor){
//    thread::spawn(move || {
//        let mut my_distributor = Some(input_distributor);
//        loop{
//            let logic_msg = logic_output.recv().unwrap();
//            match logic_msg{
//
//                LogicOutwardsMessage::DataNeeded(syncer_request) => {
////                    network_sink.send(NetMessageType::())
//                }
//                LogicOutwardsMessage::IAmInitialized() => {
//                    my_distributor.take().unwrap().start();
//                }
//            }
//        }
//    });
//}