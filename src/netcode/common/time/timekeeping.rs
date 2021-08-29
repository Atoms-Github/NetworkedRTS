use crossbeam_channel::*;
use std::thread;
use std::time::{SystemTime, Duration};
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use serde::*;

//pub const FRAME_DURATION_MILLIS: f32 = 200.0;
// pub const FRAME_DURATION_MILLIS: f32 = 100.0;
// pub const FRAME_DURATION_MILLIS: f32 = 50.0;
// pub const FRAME_DURATION_MILLIS: f32 = 30.0;
//pub const FRAME_DURATION_MILLIS: f32 = 18.0;
pub const FRAME_DURATION_MILLIS: f32 = 16.667;
//pub const FRAME_DURATION_MILLIS: f32 = 10.0;
//pub const FRAME_DURATION_MILLIS: f32 = 5.0;
//pub const FRAME_DURATION_MILLIS: f32 = 3.0; // Smaller than this things are allowed to break.

use std::ops::Add;
use std::ops::Sub;




#[derive(Serialize, Deserialize,Clone,  Debug)]
pub struct KnownFrameInfo{
    known_frame_index: FrameIndex,
    time: SystemTime
}
impl KnownFrameInfo{
    pub fn apply_offset(&mut self, offset_ns: i64){
        // TODO1: Fix this rubbish. Here's abs: let test = i64::abs(offset_ns);
        // TODO3: Find the abs function.
        if offset_ns > 0{
            self.time = self.time.add(Duration::from_nanos(offset_ns as u64));
        }else{
            self.time = self.time.sub(Duration::from_nanos(-offset_ns as u64));
        }
    }
    pub fn new_from_args(frame_index: FrameIndex, time: SystemTime) -> KnownFrameInfo{
        KnownFrameInfo{
            known_frame_index: frame_index,
            time
        }
    }
    pub fn get_intended_current_frame(&self) -> usize{
        let time_since_known_frame = SystemTime::now().duration_since(self.time).unwrap();

        self.known_frame_index + (time_since_known_frame.as_millis() as f32 / FRAME_DURATION_MILLIS).floor() as usize
    }
    pub fn start_frame_stream_from_any(&self, first_to_send: FrameIndex) -> Receiver<FrameIndex>{
        let (sender, receiver) = unbounded();
        let frame_info = self.clone();
        thread::spawn( move|| {
            // I'm gonna:
            // 1. Rewrite this.
            // 2. If it fixes it, see why.
            // 3. See why it didn't crash and fail not having the ';'.
            let mut next_frame_to_send = first_to_send;
            loop{
                let intended_frame = frame_info.get_intended_current_frame();

                while intended_frame >= next_frame_to_send {
                    sender.send(next_frame_to_send).unwrap();
                    next_frame_to_send += 1;
                }
                thread::sleep(Duration::from_millis(1))
                // modival Sleep until just before next frame to help performance.
            }
        });
        receiver
    }
    pub fn start_frame_stream_from_known(&self) -> Receiver<FrameIndex>{
        self.start_frame_stream_from_any(self.known_frame_index)
    }
    pub fn start_frame_stream_from_now(&self) -> Receiver<FrameIndex>{
        self.start_frame_stream_from_any(self.get_intended_current_frame())
    }
}
pub struct SpeedTimer{
    item: Option<DT>
}
impl SpeedTimer{
    pub fn new() -> Self{
        Self{
            item: None
        }
    }
    pub fn start(&mut self){
        if self.item.is_none(){
            self.item = Some(DT::start("testing!"));
        }
    }
    pub fn end(&mut self){
        if let Some(item) = &self.item{
            let time_since = SystemTime::now().duration_since(item.time).unwrap();
            println!("Testtimer: {:?}", time_since);
        }
        self.item = None;
    }
}
#[derive(Clone)]
pub struct DT{ // Debug Timer.
    time: SystemTime,
    pub name: String
}
impl Default for DT{
    fn default() -> Self {
        Self{
            time: SystemTime::now(),
            name: "NoNameSet".to_string()
        }
    }
}
impl DT{
    pub fn start(name: &str) -> DT{
        DT::start_fmt(String::from(name))
    }
    pub fn start_fmt(name: String) -> DT{
        DT{
            time: SystemTime::now(),
            name
        }
    }

    pub fn stop(self) -> Duration{
        let time_since = SystemTime::now().duration_since(self.time).unwrap();
        if crate::DEBUG_MSGS_TIMERS{
            log::info!("TIMER {} -> {:?}", self.name, time_since);
        }
        return time_since;
    }
    pub fn stop_fmt(self) -> String{
        let time_since = SystemTime::now().duration_since(self.time).unwrap();
        return format!("TIMER {} -> {:?}", self.name, time_since);
    }
    pub fn stop_warn(self, micro_seconds_limit: u128){

        let duration = SystemTime::now().duration_since(self.time).unwrap();
        if crate::DEBUG_MSGS_TIMERS && duration.as_micros() >= micro_seconds_limit{
            log::warn!("SLOWTIMER {} -> {:?}", self.name, duration);
        }
    }
}
