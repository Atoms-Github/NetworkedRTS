
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, TryRecvError, Sender};
use serde::{Deserialize, Serialize};

use crate::game::timekeeping::*;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_structs::*;
use std::{panic, thread};
use std::collections::HashMap;
use std::time::Duration;
use crate::game::synced_data_stream::*;
use crate::players::inputs::*;
use crate::game::bonus_msgs_segment::*;
use crate::game::logic::logic_data_storage::*;

pub const HEAD_AHEAD_FRAME_COUNT: usize = 20;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogicInwardsMessage {
    SyncerBonusUpdate(SyncerData<Vec<BonusEvent>>),
    SyncerInputsUpdate(SyncerData<InputState>),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogicOutwardsMessage {
    DataNeeded(SyncerRequestTyped),
    IAmInitialized()
}

pub struct LogicSegmentTailer {
    known_frame_info: KnownFrameInfo,
    game_state_tail: Arc<RwLock<GameState>>,
    all_frames: Arc<RwLock<LogicDataStorage>>,
    outwards_messages: Sender<LogicOutwardsMessage>
    // Logic layer shouldn't know it's player ID.
}


impl LogicSegmentTailer {
    pub fn new(known_frame_info: KnownFrameInfo, state_tail: Arc<RwLock<GameState>>,
               outwards_messages: Sender<LogicOutwardsMessage>, data_store: Arc<RwLock<LogicDataStorage>>) -> LogicSegmentTailer {
        LogicSegmentTailer {
            known_frame_info,
            game_state_tail: state_tail,
            all_frames: data_store,
            outwards_messages
        }
    }

    fn sim_tail_frame(&mut self, tail_frame_to_sim: FrameIndex) -> Vec<SyncerRequestTyped>{
        let sim_query_result;
        {
            sim_query_result = self.all_frames.read().unwrap().clone_info_for_sim(tail_frame_to_sim);
        }
        if sim_query_result.missing_info.len() > 0{
            return sim_query_result.missing_info;
        }
        {
            // It's fine to hold the state for a while as this thread is important - and we shouldn't be long in comparison to head.
            self.game_state_tail.write().unwrap().simulate_tick(&sim_query_result.sim_info, FRAME_DURATION_MILLIS);
        }

        return None; // No missing frames.
    }


    pub fn start_logic_thread(mut self){
        let mut generator = self.known_frame_info.start_frame_stream_from_any(self.game_state_tail.get_simmed_frame_index());
        thread::spawn(move ||{
            let mut my_self = self;
            loop{
                let tail_frame_to_sim = generator.recv().unwrap();

                loop{
                    let problems = my_self.sim_tail_frame(tail_frame_to_sim);
                    if problems.len() > 0 {
                        break;
                    }

                    for problem in problems{
                        println!("Logic missing info so asking: {:?}", problem);
                        my_self.outwards_messages.send( LogicOutwardsMessage::DataNeeded(problem) ).unwrap();


                    }
                    std::thread::sleep(Duration::from_millis(100)); // Wait to save CPU cycles. modival: Can optimise recovery time by increasing check rate, but resend rate shouldn't be too high cos can't have too many messages.

                }
            }
        });
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
//NetMessageType::ConnectionInit(msg_init) => {
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
//NetMessageType::InputsUpdate(msg_inputs) => {
//for online_player in &mut self.online_players{
//online_player.controller = msg_inputs.controllers[0].clone();
//}
//
//},
//};
//}
//messages_this_frame.clear();
//}
