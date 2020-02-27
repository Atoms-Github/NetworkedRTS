

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

pub struct BonusMsgsSegment{
    known_frame: KnownFrameInfo,
    bonus_msgs_frames: Vec<Vec<BonusEvent>>
}

impl BonusMsgsSegment{
    pub fn new(known_frame: KnownFrameInfo) -> BonusMsgsSegment{
        BonusMsgsSegment{
            known_frame,
            bonus_msgs_frames : Vec::new()
        }
    }
    pub fn start(self) -> Receiver<BonusMsgsResponse>{
        let (out_msgs_sink, out_msgs_rec) = channel();
        thread::spawn(move ||{
            let new_frame_o_matic = self.known_frame.start_frame_stream();
            loop{
                let frame_index = new_frame_o_matic.recv().unwrap();

                let bonus_events = BonusMsgsResponse{
                    start_frame_index: frame_index,
                    event_lists: vec![vec![]]
                };
                out_msgs_sink.send(bonus_events).unwrap();
            }
        });
        return out_msgs_rec;
    }
}




