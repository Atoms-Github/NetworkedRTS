
use std::collections::HashMap;
use std::{panic, thread};

use serde::{Deserialize, Serialize};

use crate::game::bonus_msgs_segment::*;
use crate::game::logic::logic_segment::*;
use crate::game::synced_data_stream::*;
use crate::network::game_message_types::NewPlayerInfo;
use crate::network::networking_structs::*;
use crate::players::inputs::*;

use crate::game::logic::logic_data_storage::*;
use std::sync::{Mutex, Arc, RwLock};
use std::time::Duration;
use crate::game::timekeeping::FRAME_DURATION_MILLIS;


pub struct DataStorageManager<T>{
    pub value: Arc<RwLock<T>> // Yup, that's it.
}
// TODO1: Add apply game messages section.

//impl<T> DataStorageManager<T>{
//    pub fn get_read_mutex(&self){
//        return self.read_only.clone();
//    }
//
//    pub fn modify_data(){
//
//    }
//
//    pub fn start_thread(){
//        thread::spawn(||{
//            loop{
//                thread::sleep(Duration::from_millis((FRAME_DURATION_MILLIS / 2.0) as u64));
//            }
//        });
//    }
//}
