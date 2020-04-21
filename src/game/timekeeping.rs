use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{SystemTime, Duration};

use serde::*;

use crate::network::networking_structs::FrameIndex;

pub const FRAME_DURATION_MILLIS: f64 = 100.0;
//pub const FRAME_DURATION_MILLIS: f64 = 16.66;




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
    pub fn new_from_current_time(frame_reference: &KnownFrameInfo) -> Self{
        KnownFrameInfo{
            known_frame_index: frame_reference.get_intended_current_frame(),
            time: SystemTime::now()
        }
    }
    pub fn get_intended_current_frame(&self) -> usize{
        let time_since_known_frame = SystemTime::now().duration_since(self.time).unwrap();

        let intended_frame = self.known_frame_index + (time_since_known_frame.as_millis() as f64 / FRAME_DURATION_MILLIS).floor() as usize;
        return intended_frame;
    }
    pub fn start_frame_stream_from_any(&self, first_to_send: FrameIndex) -> Receiver<FrameIndex>{
        let (sender, receiver) = channel();
        let frame_info = self.clone();
        // TODO2: Remove the +1 -1 and such.
        thread::spawn( move|| {
            let sink = sender;

            let mut last_frame_simed: i32 = first_to_send as i32 - 1 ;
            loop{
                let intended_frame = frame_info.get_intended_current_frame() as i32;
                if last_frame_simed < intended_frame {
                    last_frame_simed += 1;
                    sink.send(last_frame_simed as usize).unwrap();
                }
                thread::sleep(Duration::from_millis(1))
                // modival Sleep until just before next frame to help performance.
                // TODO2: Use a clever sleep amount.

            }
        });
        return receiver;
    }
    pub fn start_frame_stream_from_known(&self) -> Receiver<FrameIndex>{
        self.start_frame_stream_from_any(self.known_frame_index)
    }
}