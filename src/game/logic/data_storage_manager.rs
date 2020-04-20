use std::{thread};

use serde::{Deserialize, Serialize};

use crate::game::logic::logic_segment::*;


use crate::game::logic::logic_data_storage::*;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Receiver, Sender, channel};



pub struct DataStorageManagerEx {
    pub logic_msgs_sink: Sender<LogicInwardsMessage>,
    pub data_lock: Arc<RwLock<LogicDataStorage>> // Yup, that's it.
}
impl DataStorageManagerEx {
    pub fn clone_lock_ref(&self) -> Arc<RwLock<LogicDataStorage>>{
        return self.data_lock.clone();
    }
}
pub struct DataStorageManagerIn {
    data_storage: LogicDataStorage
}

impl DataStorageManagerIn {
    pub fn new(storage: LogicDataStorage) -> DataStorageManagerIn{
        return DataStorageManagerIn {
            data_storage: storage
        }
    }
    pub fn init_data_storage(self) -> DataStorageManagerEx{
        let (logic_inwards_sink, logic_inwards_rec) = channel();
        let lock = Arc::new(RwLock::new(self.data_storage));

        DataStorageManagerIn::start_thread(lock.clone(), logic_inwards_rec);
        return DataStorageManagerEx{
            logic_msgs_sink: logic_inwards_sink,
            data_lock: lock
        }
    }

    fn start_thread(lock: Arc<RwLock<LogicDataStorage>>, inputs_channel: Receiver<LogicInwardsMessage>) {
        let lock_clone = lock.clone();
        thread::spawn(move || {
            loop {
                let next_msg = inputs_channel.recv().unwrap();
                lock_clone.write().unwrap().handle_inwards_msg(next_msg);
            }
        });
    }
}

