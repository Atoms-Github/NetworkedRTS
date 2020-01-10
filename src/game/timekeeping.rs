use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{SystemTime};

use serde::*;

use crate::network::networking_structs::FrameIndex;

pub const FRAME_DURATION: f64 = 0.0166;


pub struct SimableFrameInfo {
    pub frame_index: FrameIndex,
    pub delta: f32
}


#[derive(Serialize, Deserialize,Clone,  Debug)]
pub struct KnownFrameInfo{
    pub known_frame_index: FrameIndex,
    pub time: SystemTime
}

//pub struct SimableFrameGenerator{
//    frame_stream: Receiver<FrameIndex>
//}
//impl SimableFrameGenerator{
//    pub fn recv(&mut self) -> KnownFrameInfo{
//        let frame_number = self.recv();
//
//
//
//        KnownFrameInfo{
//            known_frame_index: 0,
//            time: ()
//        }
//    }
//    pub fn start_new_generator(frame_info: KnownFrameInfo) -> SimableFrameGenerator{
//        let stream = frame_info.start_frame_stream();
//        SimableFrameGenerator{
//            frame_stream: stream
//        }
//    }
//}

impl KnownFrameInfo{
    pub fn get_intended_current_frame(&self) -> usize{
        let time_since_known_frame = SystemTime::now().duration_since(self.time).unwrap();

        let intended_frame = self.known_frame_index + (time_since_known_frame.as_micros() as f64 / FRAME_DURATION).floor() as usize;
        return intended_frame;
    }

    pub fn start_frame_stream(&self) -> Receiver<FrameIndex>{
        let (sender, receiver) = channel();
        let frame_info = self.clone();
        // TODO: find how to close thread when not needed.
        thread::spawn( move|| {
            let sink = sender;

            let mut last_frame_simed = frame_info.known_frame_index - 1;
            loop{
                let intended_frame = frame_info.get_intended_current_frame();
                if last_frame_simed < intended_frame {
                    last_frame_simed += 1;
                    sink.send(last_frame_simed);
                }
                // TODO: Sleep until just before next frame to help performance.
            }
        });
        return receiver;
    }
}