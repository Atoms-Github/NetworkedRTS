use std::panic;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration};

use crate::common::logic::logic_sim_tailer_seg::*;
use crate::common::network::external_msg::*;
use crate::common::time::timekeeping::*;





pub struct NetRecSegIn{
    incoming_msgs: Vec<i32>,
    net_inc: Receiver<ExternalMsg>,
    known_frame: KnownFrameInfo,
    to_logic: Sender<LogicInwardsMessage>,
}
impl NetRecSegIn{
    fn pull_from_net(&mut self){

    }
    fn apply_to_store(&mut self){

    }
    fn get_duration_before_frame(&self) -> Duration{
        let milis;
        if FRAME_DURATION_MILLIS >= 5.0 {
            milis = FRAME_DURATION_MILLIS - 2.0; // Should manage calculations in 2ms.
        }else{
            milis = FRAME_DURATION_MILLIS / 5.0 * 4.0; // Probably not going to manage calculations - don't care about timing, just don't want to crash.
        }
        Duration::from_secs_f32(milis / 1000.0)
    }
    pub fn new(to_logic: Sender<LogicInwardsMessage>, net_inc: Receiver<ExternalMsg>, known_frame: KnownFrameInfo) -> Self{
        Self{
            incoming_msgs: vec![],
            net_inc,
            known_frame,
            to_logic
        }
    }
    pub fn start(mut self) -> NetRecSegEx{
        thread::spawn(move || {
            let frame_counter = self.known_frame.start_frame_stream_from_now();
            loop{

                // TODO3: Can implement a thread thinggy wrapper around the storage to ensure things aren't waiting for each other. (Implement msg gathering)
//                frame_counter.recv().unwrap(); // Just for timing - don't care of frame index.
//                thread::sleep(self.get_duration_before_frame()); // Wait until just before next frame.

                match self.net_inc.recv().unwrap(){
                    ExternalMsg::GameUpdate(update) => {
                        if crate::DEBUG_MSGS_MAIN {
                            println!("Net rec message: {:?}", update);
                        }
                        self.to_logic.send(update).unwrap();
                    },
                    ExternalMsg::LocalCommand(_) => {panic!("Not implemented!")},
                    ExternalMsg::PingTestResponse(_) => {
                        // Do nothing. Doesn't matter that intro stuff is still floating when we move on.
                    }
                    _ => {
                        panic!("Client shouldn't be getting a message of this type (or at this time)!")
                    }
                }
            }
        });

        NetRecSegEx{

        }
    }
}

pub struct NetRecSegEx{

}
impl NetRecSegEx{

}