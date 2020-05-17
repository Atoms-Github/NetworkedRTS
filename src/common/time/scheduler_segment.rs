use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::common::time::timekeeping::*;
use crate::common::types::*;

pub struct SchedulerSegEx {
    trigger_sink: Sender<ScheduledTrigger>
}
struct ScheduledTrigger {
    frame_at: FrameIndex,
    event: Box<dyn FnOnce() + Send> // Could be another type of function if needed.
}

pub struct SchedulerSegIn {
    known_frame: KnownFrameInfo,
    triggers: Vec<ScheduledTrigger>
//    player_id: PlayerID
}
impl SchedulerSegIn {
    pub fn new(known_frame: KnownFrameInfo) -> SchedulerSegIn {
        return SchedulerSegIn {
            known_frame,
            triggers: vec![]
        }
    }
    fn pull_latest_triggers(&mut self, trigger_stream: &mut Receiver<ScheduledTrigger>){
        loop{
            let pulled = trigger_stream.try_recv();
            if pulled.is_ok(){
                self.triggers.push(pulled.unwrap());
            }else{
                break;
            }
        }
    }
    pub fn start(mut self) -> SchedulerSegEx{
        let (mut trigger_sink, mut trigger_rec) = channel();
        thread::spawn(move ||{
            let gen = self.known_frame.start_frame_stream_from_known();
            loop{
                let now_frame = gen.recv().unwrap();
                self.pull_latest_triggers(&mut trigger_rec);
                for trigger in self.triggers.drain_filter(|t|{
                    return t.frame_at == now_frame;
                }){
                    (trigger.event)();
                }

            }
        });
        return SchedulerSegEx{
            trigger_sink
        }
    }
}
impl SchedulerSegEx {
    pub fn schedule_event(&mut self, event: Box<dyn FnOnce() + Send>, frame: FrameIndex){
        self.trigger_sink.send(
            ScheduledTrigger{
                frame_at: frame,
                event
            }
        ).unwrap()
    }
}