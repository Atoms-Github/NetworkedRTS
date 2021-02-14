use crossbeam_channel::*;
use std::thread;

use crate::netcode::common::time::timekeeping::*;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;

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
    fn pull_latest_triggers(&mut self, trigger_stream: &mut Receiver<ScheduledTrigger>){
        loop{
            let pulled = trigger_stream.try_recv();
            match pulled{
                Ok(trigger) => {
                    self.triggers.push(trigger);
                }
                Err(error) => {
                    break;
                }
            }
        }
    }
    pub fn start(mut self) -> SchedulerSegEx{
        let (mut trigger_sink, mut trigger_rec) = unbounded();
        thread::spawn(move ||{
            let gen = self.known_frame.start_frame_stream_from_known();
            loop{
                let now_frame = gen.recv().unwrap();
                self.pull_latest_triggers(&mut trigger_rec);
                for trigger in self.triggers.drain_filter(|t|{
                    t.frame_at == now_frame
                }){
                    (trigger.event)();
                }

            }
        });
        SchedulerSegEx{
            trigger_sink
        }
    }
}
impl SchedulerSegEx {
    pub fn start(known_frame: KnownFrameInfo) -> Self{
        SchedulerSegIn {
            known_frame,
            triggers: vec![]
        }.start()
    }
    pub fn schedule_event(&self, event: Box<dyn FnOnce() + Send>, frame: FrameIndex){
        self.trigger_sink.send(
            ScheduledTrigger{
                frame_at: frame,
                event
            }
        ).unwrap()
    }
}