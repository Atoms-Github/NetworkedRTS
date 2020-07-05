use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Receiver, Sender};

use serde::{Deserialize, Serialize};

use crate::common::gameplay::game::game_state::*;
use crate::common::sim_data::framed_vec::*;
use crate::common::sim_data::input_state::*;
use crate::common::sim_data::sim_data_storage::*;
use crate::common::time::timekeeping::*;
use crate::common::types::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogicInwardsMessage {
    SyncerInputsUpdate(FramedVecDataPack<InputState>),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogicOutwardsMessage {
    DataNeeded(FramedVecRequestTyped),
    IAmInitialized()
}

pub struct LogicSimTailerEx {
    pub from_logic_rec: Receiver<LogicOutwardsMessage>,
    pub tail_lock: Arc<RwLock<GameState>>,
    pub new_tail_states_rec: Option<Receiver<GameState>>,

}
impl LogicSimTailerEx {

}
pub struct LogicSegmentTailerIn {
    known_frame_info: KnownFrameInfo,
    tail_lock: Arc<RwLock<GameState>>,
    all_frames: Arc<RwLock<SimDataStorage>>
    // Logic layer shouldn't know it's player ID.
}

impl LogicSegmentTailerIn {
    pub fn new(known_frame_info: KnownFrameInfo, state_tail: GameState,
               data_store: Arc<RwLock<SimDataStorage>>) -> LogicSegmentTailerIn {
        LogicSegmentTailerIn {
            known_frame_info,
            tail_lock: Arc::new(RwLock::new(state_tail)),
            all_frames: data_store
        }
    }

    fn try_sim_tail_frame(&mut self, tail_frame_to_sim: FrameIndex) -> Vec<FramedVecRequestTyped>{

        let sim_query_result;
        {
            sim_query_result = self.all_frames.read().unwrap().clone_info_for_sim(tail_frame_to_sim);
        }
        if !sim_query_result.missing_info.is_empty(){
            return sim_query_result.missing_info;
        }
        // It's fine to hold the state for a while as this thread is important - and we shouldn't be long in comparison to head.
        {
            let mut state_handle = self.tail_lock.write().unwrap();
            state_handle.simulate_tick(sim_query_result.sim_info, FRAME_DURATION_MILLIS);
            println!("TailSim {}", state_handle.get_simmed_frame_index());
        }


        return vec![]; // No missing frames.
    }


    fn start_thread(mut self, outwards_messages: Sender<LogicOutwardsMessage>, mut new_tails_sink: Sender<GameState>){
        thread::spawn(move ||{
            let first_frame_to_sim = self.tail_lock.read().unwrap().get_simmed_frame_index() + 1;
            println!("Logic got state. Next frame to sim: {}", first_frame_to_sim);
            let mut generator = self.known_frame_info.start_frame_stream_from_any(first_frame_to_sim);
//            let (execution_sink, execution_rec) = channel();
            let mut frame_to_sim = first_frame_to_sim;
            loop{
                let frame_to_sim_if_no_problems = generator.recv().unwrap();

                while frame_to_sim < frame_to_sim_if_no_problems{ // Try to catch up as much as possible.
                    let problems = self.try_sim_tail_frame(frame_to_sim);
                    if problems.is_empty() {
                        frame_to_sim += 1;
                    }else{
                        for problem in &problems{
                            println!("Logic missing info so asking: {:?}", problem);
                            outwards_messages.send( LogicOutwardsMessage::DataNeeded(problem.clone()) ).unwrap();
                        }
                        break; // No more catch up possible without info.
                    }
                }
                new_tails_sink.send(self.tail_lock.read().unwrap().clone()).unwrap(); // Send new head regardless of success.

            }
        });
    }

    pub fn start_logic_tail(mut self) -> LogicSimTailerEx {
        let (from_logic_sink, from_logic_rec) = channel();
        let (new_tails_sink, new_tails_rec) = channel();


        let tail_lock = self.tail_lock.clone();

        self.start_thread(from_logic_sink, new_tails_sink);

        LogicSimTailerEx {
            from_logic_rec,
            tail_lock,
            new_tail_states_rec: Some(new_tails_rec)
        }
    }
}



//let controllers = self.get_controllers_clone();
//
//let mut pending = PendingEntities::new();
//
//secret_position_system(&self.world, &mut pending, &mut self.storage.position_s, &mut self.storage.velocity_s);
//secret_velocity_system(&self.world, &mut pending, &mut self.storage.position_s, &mut self.storage.velocity_s);
//secret_velocityWithInput_system(&self.world, &mut pending, &mut self.storage.velocity_s,
//&mut self.storage.velocityWithInput_s, &controllers);
//
//self.world.update_entities(&mut self.storage, pending);



//
////            std::mem::drop();
//{ // Need to be explicit in where the mutex locks are dropped.
//let mut messages_this_frame = self.messages_to_process.lock().unwrap();
//for net_message in &*messages_this_frame{
//match net_message{
//ExternalMsg::ConnectionInit(msg_init) => {
//println!("Welcomed with a message: {}", msg_init.welcome_msg);
//self.online_players.push(OnlinePlayer{
//controller: PlayerController { input_state: InputState::new()}
//});
//
//
//let mut pending = PendingEntities::new();
//
//
//let mut pending_entity_online_player = PendingEntity::new();
//pending_entity_online_player.add_component(PositionComp{ x: 0.0, y: 0.0 });
//pending_entity_online_player.add_component(VelocityComp{ x: 2.0, y: 1.0 });
//pending_entity_online_player.add_component(SizeComp{ x: 50.0, y: 50.0 });
//pending_entity_online_player.add_component(velocityWithInputComp{ owner_id: 2 });
//pending_entity_online_player.add_component(RenderComp{ hue: graphics::Color::from_rgb(255,150,150) });
//pending.create_entity(pending_entity_online_player);
//
//
//
//self.world.update_entities(&mut self.storage, pending);
//
//
//
//},
//ExternalMsg::InputsUpdate(msg_inputs) => {
//for online_player in &mut self.online_players{
//online_player.controller = msg_inputs.controllers[0].clone();
//}
//
//},
//};
//}
//messages_this_frame.clear();
//}
