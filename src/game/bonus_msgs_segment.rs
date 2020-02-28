

use std::net::SocketAddr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{SystemTime};

use crate::game::logic_segment::LogicSegment;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_hub_segment::{DistributableNetMessage, NetworkingHub, OwnedNetworkMessage};
use crate::network::networking_structs::*;
use crate::network::networking_message_types::*;
use crate::network::game_message_types::*;
use std::sync::{Mutex, Arc};
use std::thread;
use crate::network::game_message_types::LogicInwardsMessage;
use crate::network::game_message_types::LogicOutwardsMessage;
use std::panic;

struct NewBonusEvent{
    bonus_event: BonusEvent,
    execution_frame: FrameIndex,
}
pub struct BonusMsgsSegment{
    known_frame: KnownFrameInfo,
    bonus_msgs_frames: Vec<Vec<BonusEvent>>,
    new_bonus_events: Vec<NewBonusEvent>,
}

impl BonusMsgsSegment{
    pub fn new(known_frame: KnownFrameInfo) -> BonusMsgsSegment{
        BonusMsgsSegment{
            known_frame,
            bonus_msgs_frames : Vec::new(),
            new_bonus_events: vec![]
        }
    }
    pub fn start(mut self) -> Receiver<BonusMsgsResponse>{
        let (out_msgs_sink, out_msgs_rec) = channel();
        if self.bonus_msgs_frames.len() > 0{
            panic!();
        }
        thread::Builder::new().name("BonusMsgsMain".to_string()).spawn(move ||{
            let new_frame_o_matic = self.known_frame.start_frame_stream();
            loop{
                let frame_index = new_frame_o_matic.recv().unwrap();

                for forecast_frame_index in (self.bonus_msgs_frames.len() + 1)..(frame_index + 60){
                    let mut new_event_list = vec![];
                    let matching_new_events = self.new_bonus_events.drain_filter(|potentially_matching_new_event|{
                        return potentially_matching_new_event.execution_frame == forecast_frame_index;
                    });
                    for new_event_in_frame in matching_new_events{
                        new_event_list.push(new_event_in_frame.bonus_event);
                    }

                    let bonus_events = BonusMsgsResponse{
                        start_frame_index: forecast_frame_index,
                        event_lists: vec![new_event_list.clone()]
                    };
                    out_msgs_sink.send(bonus_events).unwrap();

                    self.bonus_msgs_frames.push(new_event_list); // Save to history.
                }
            }
        }).unwrap();
        return out_msgs_rec;
    }
}




