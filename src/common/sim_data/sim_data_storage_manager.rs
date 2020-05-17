use std::{thread};


use crate::common::logic::logic_sim_tailer_seg::*;
use crate::common::sim_data::sim_data_storage::*;

use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Receiver, Sender, channel};



pub struct SimDataStorageManagerEx {
    pub logic_msgs_sink: Sender<LogicInwardsMessage>,
    pub data_lock: Arc<RwLock<SimDataStorage>> // Yup, that's it.
}
impl SimDataStorageManagerEx {
    pub fn clone_lock_ref(&self) -> Arc<RwLock<SimDataStorage>>{
        return self.data_lock.clone();
    }
}
pub struct SimDataStorageManagerIn {
    data_storage: SimDataStorage
}

impl SimDataStorageManagerIn {
    pub fn new(storage: SimDataStorage) -> SimDataStorageManagerIn{
        return SimDataStorageManagerIn {
            data_storage: storage
        }
    }
    pub fn init_data_storage(self) -> SimDataStorageManagerEx{
        let (logic_inwards_sink, logic_inwards_rec) = channel();
        let lock = Arc::new(RwLock::new(self.data_storage));

        SimDataStorageManagerIn::start_thread(lock.clone(), logic_inwards_rec);
        return SimDataStorageManagerEx{
            logic_msgs_sink: logic_inwards_sink,
            data_lock: lock
        }
    }

    fn start_thread(lock: Arc<RwLock<SimDataStorage>>, inputs_channel: Receiver<LogicInwardsMessage>) {
        let lock_clone = lock.clone();
        thread::spawn(move || {
            loop {
                let next_msg = inputs_channel.recv().unwrap();
                lock_clone.write().unwrap().handle_inwards_msg(next_msg);
            }
        });
    }
}

