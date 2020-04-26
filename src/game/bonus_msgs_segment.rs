use std::panic;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use serde::*;

use crate::game::synced_data_stream::*;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_structs::*;
use std::collections::HashMap;

const BONUS_FRAMES_AHEAD: usize = 40; // modival

struct TimedBonusEvent {
    bonus_event: BonusEvent,
    execution_frame: Option<FrameIndex>,
}
pub struct BonusMsgsSegmentIn {
    known_frame: KnownFrameInfo,
    bonus_msgs_frames: Vec<Vec<BonusEvent>>, // If we do eventually switch to udp we can expose the list of all events history.
//    new_bonus_events: Vec<NewBonusEvent>,
    events_map: HashMap<FrameIndex, Vec<BonusEvent>>,
}
pub struct BonusMsgsSegmentEx {
    pub scheduled_events: Option<Receiver<SyncerData<Vec<BonusEvent>>>>,
    event_dump: Sender<TimedBonusEvent>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BonusEvent{
    NewPlayer(PlayerID),
}

impl BonusMsgsSegmentEx{
    pub fn schedule_event(&mut self, event: BonusEvent){
        let event = TimedBonusEvent{
            bonus_event: event,
            execution_frame: None
        };
        self.event_dump.send(event).unwrap();
    }
    pub fn schedule_event_timed(&mut self, event: BonusEvent, frame_index: FrameIndex){
        let event = TimedBonusEvent{
            bonus_event: event,
            execution_frame: Some(frame_index)
        };
        self.event_dump.send(event).unwrap();
    }
}

impl BonusMsgsSegmentIn {
    pub fn new(known_frame: KnownFrameInfo) -> BonusMsgsSegmentIn {
        BonusMsgsSegmentIn {
            known_frame,
            bonus_msgs_frames : vec![],
            events_map: Default::default()
        }
    }
    fn get_new_events_on_frame(&mut self, frame_index: FrameIndex) -> Vec<BonusEvent>{
        let mut events_found = vec![];
        let events_on_frame = self.events_map.get_mut(&frame_index);
        if events_on_frame.is_some(){
            let found = events_on_frame.unwrap();
            events_found.append(found);
        }
        return events_found;
    }
    fn send_events_from_frame(&mut self, scheduled_sink: &Sender<SyncerData<Vec<BonusEvent>>>, frame_index: FrameIndex){
        let events_on_frame = self.get_new_events_on_frame(frame_index);
        let data = SyncerData{
            data: vec![events_on_frame],
            start_frame: frame_index,
            owning_player: -1
        };
        if crate::SEND_DEBUG_MSGS{
            println!("Sending bonus: {:?}", data);
        }
        scheduled_sink.send(data).unwrap();

    }
    fn start_bonus_thread(mut self, to_schedule_rec: Receiver<TimedBonusEvent>, schedled_sink: Sender<SyncerData<Vec<BonusEvent>>>){
        thread::Builder::new().name("BonusMsgsMain".to_string()).spawn(move ||{

            self.add_new_events_to_map(&to_schedule_rec);
            for setup_frame_index in 0 .. BONUS_FRAMES_AHEAD{
                self.send_events_from_frame(&schedled_sink, setup_frame_index);
            }

            let new_frame_o_matic = self.known_frame.start_frame_stream_from_any(0);
            loop{
                let frame_index = new_frame_o_matic.recv().unwrap() + BONUS_FRAMES_AHEAD;
                self.add_new_events_to_map(&to_schedule_rec);
                self.send_events_from_frame(&schedled_sink, frame_index);
            }
        }).unwrap();
    }
    fn add_event_to_map(&mut self, event: BonusEvent, schedule_frame: FrameIndex){
        let processing_frame = self.known_frame.get_intended_current_frame();
        if schedule_frame - processing_frame < 3{
            panic!("Tried to schedule a bonus event that was too near to the present!");
        }
        if !self.events_map.contains_key(&schedule_frame){
            self.events_map.insert(schedule_frame, vec![]);
        }
        self.events_map.get_mut(&schedule_frame).unwrap().push(event);
    }
    fn add_new_events_to_map(&mut self, in_msgs_rec: &Receiver<TimedBonusEvent>) {
        let mut to_schedule = in_msgs_rec.try_recv();
        while to_schedule.is_ok(){
            let timed_bonus = to_schedule.unwrap();
            let future_frame = timed_bonus.execution_frame.unwrap_or(self.known_frame.get_intended_current_frame() + 60);
            self.add_event_to_map(timed_bonus.bonus_event, future_frame);
            to_schedule = in_msgs_rec.try_recv();
        }
    }
    pub fn start(mut self) -> BonusMsgsSegmentEx{
        let (out_msgs_sink, out_msgs_rec) = channel(); // Messages that have been scheduled.
        let (in_msgs_sink, in_msgs_rec) = channel(); // Messages to schedule somewhere.
        if self.bonus_msgs_frames.len() > 0{
            panic!();
        }
        self.start_bonus_thread(in_msgs_rec, out_msgs_sink);
        return BonusMsgsSegmentEx{
            scheduled_events: Some(out_msgs_rec),
            event_dump: in_msgs_sink,
        };
    }

}




