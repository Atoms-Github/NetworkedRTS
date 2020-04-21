use std::panic;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use serde::*;

use crate::game::synced_data_stream::*;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_structs::*;

struct NewBonusEvent{
    bonus_event: BonusEvent,
    execution_frame: FrameIndex,
}
pub struct BonusMsgsSegmentIn {
    known_frame: KnownFrameInfo,
    bonus_msgs_frames: Vec<Vec<BonusEvent>>,
    new_bonus_events: Vec<NewBonusEvent>,
}
pub struct BonusMsgsSegmentEx {
    pub scheduled_events: Receiver<SyncerData<Vec<BonusEvent>>>,
    pub event_dump: Sender<ScheduledBonusEvent>
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BonusEvent{
    NewPlayer(PlayerID),
    None
}
pub struct ScheduledBonusEvent{
    event: BonusEvent,
    when_frame: FrameIndex
}

impl BonusMsgsSegmentEx{

}

impl BonusMsgsSegmentIn {
    pub fn new(known_frame: KnownFrameInfo) -> BonusMsgsSegmentIn {
        BonusMsgsSegmentIn {
            known_frame,
            bonus_msgs_frames : Vec::new(),
            new_bonus_events: vec![]
        }
    }
    pub fn start(mut self) -> BonusMsgsSegmentEx{
        let (out_msgs_sink, out_msgs_rec) = channel(); // Messages that have been scheduled.
        let (in_msgs_sink, in_msgs_rec) = channel(); // Messages to schedule somewhere.
        if self.bonus_msgs_frames.len() > 0{
            panic!();
        }
        thread::Builder::new().name("BonusMsgsMain".to_string()).spawn(move ||{
            let new_frame_o_matic = self.known_frame.start_frame_stream_from_known();
            loop{
                let frame_index = new_frame_o_matic.recv().unwrap();

                self.read_new_events(&in_msgs_rec, frame_index);
                
                for forecast_frame_index in (self.bonus_msgs_frames.len() + 1)..(frame_index + 60){
                    let mut new_event_list = vec![];
                    let matching_new_events = self.new_bonus_events.drain_filter(|potentially_matching_new_event|{
                        return potentially_matching_new_event.execution_frame == forecast_frame_index;
                    });
                    for new_event_in_frame in matching_new_events{
                        new_event_list.push(new_event_in_frame.bonus_event);
                    }
                    let bonus_events = SyncerData{
                        data: vec![new_event_list.clone()],
                        start_frame: forecast_frame_index,
                        owning_player: 0
                    };
                    out_msgs_sink.send(bonus_events).unwrap();

                    self.bonus_msgs_frames.push(new_event_list); // Save to history.
                }
            }
        }).unwrap();
        return BonusMsgsSegmentEx{
            scheduled_events: out_msgs_rec,
            event_dump: in_msgs_sink, // TODO1: Finish allowing for scheduling wherever wanted.
        };
    }
    fn read_new_events(&mut self, in_msgs_rec: &Receiver<BonusEvent>, frame_index: usize) {
        loop {
            let requested_to_schedule = in_msgs_rec.try_recv();
            match requested_to_schedule {
                Ok(bonus) => {
                    self.new_bonus_events.push(NewBonusEvent {
                        bonus_event: bonus,
                        execution_frame: frame_index + 70
                    })
                }
                Err(error) => {
                    break;
                }
            }
        }
    }
}




