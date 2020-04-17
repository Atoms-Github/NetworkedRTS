
use std::{thread};

use serde::{Deserialize, Serialize};

use crate::game::logic::logic_segment::*;


use crate::game::logic::logic_data_storage::*;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Receiver;


pub struct DataStorageManager{
    value: Arc<RwLock<LogicDataStorage>> // Yup, that's it.
}
impl DataStorageManager{
    pub fn new(storage: LogicDataStorage) -> Self{
        return DataStorageManager{
            value: Arc::new(RwLock::new(storage))
        }
    }
    pub fn clone_lock_ref(&self) -> Arc<RwLock<LogicDataStorage>>{
        return self.value.clone();
    }
    pub fn start_data_update_consumption_thread(&self, inputs_channel: Receiver<LogicInwardsMessage>){
        let my_data_handle = self.value.clone();
        thread::spawn(move ||{
            loop{
                let next_msg = inputs_channel.recv().unwrap();
                my_data_handle.write().unwrap().handle_inwards_msg(next_msg);
            }
        });
    }



}
