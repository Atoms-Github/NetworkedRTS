use std::net::{SocketAddr, TcpStream};
use std::ops::Add;
use std::str::FromStr;
use crossbeam_channel::*;
use std::thread;
use std::time::{Duration, SystemTime};
use std::ops::Div;
use std::ops::Sub;
use serde::{Deserialize, Serialize};
use crate::netcode::common::network::external_msg::*;
use std::sync::{RwLock, Arc, RwLockWriteGuard, Mutex};
use std::collections::vec_deque::*;
use std::io::Seek;
use crate::netcode::client::input_handler_seg::*;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SuperstoreData<T> {
    pub data: Vec<T>,
    pub frame_offset: FrameIndex
}

struct SuperstoreIn<T:Clone + Default + Send +  Eq + std::fmt::Debug + Sync + 'static>{
    frame_offset: usize,
    lukewarm: ArcRw<Vec<T>>,
    write_requests_rec: Receiver<SuperstoreData<T>>,
}
#[derive()]
pub struct SuperstoreEx<T:Clone + Default + Send +  Eq + std::fmt::Debug + Sync + 'static>{
    frame_offset: usize,
    lukewarm: ArcRw<Vec<T>>,
    pub write_requests_sink: Mutex<Sender<SuperstoreData<T>>>,
}




impl<T:Clone + Default + Send +  Eq + std::fmt::Debug + Sync + 'static> SuperstoreEx<T>{
    pub fn start(frame_offset: usize)-> Self{
        let (writes_sink, writes_rec) = unbounded();
        let lukewarm = Arc::new(RwLock::new(vec![]));
        SuperstoreIn{
            frame_offset,
            write_requests_rec: writes_rec,
            lukewarm: lukewarm.clone(),
        }.start();

        Self{
            frame_offset,
            lukewarm,
            write_requests_sink: Mutex::new(writes_sink)
        }
    }
    pub fn get_first_frame_index(&self) -> FrameIndex{
        return self.frame_offset;
    }
    pub fn get_clone(&self, abs_index: FrameIndex) -> Option<T>{ // optimum Shouldn't need to clone so much. Should be possible never cloning.
        // Just return none. assert!(abs_index >= self.frame_offset, format!("Tried to get data from before superstore start. TargetIndex: {}, FirstDataAt: {}", abs_index, self.frame_offset));

        if abs_index < self.frame_offset{
            return None;
        }

        let relative_index =  abs_index - self.frame_offset;
        let lukewarm = self.lukewarm.read().unwrap();

        return lukewarm.get(relative_index).cloned();
    }
    pub fn get_last_clone(&self) -> Option<T>{
        let lukewarm = self.lukewarm.read().unwrap();
        return lukewarm.last().cloned();
    }
    pub fn get_next_dataless_frame_index(&self) -> FrameIndex{
        let lukewarm = self.lukewarm.read().unwrap();
        return self.frame_offset + lukewarm.len();
    }
    fn test_set_simple(&self, data: T, frame_index: FrameIndex){
        self.write_requests_sink.lock().unwrap().send(SuperstoreData{
            data: vec![data],
            frame_offset: frame_index
        }).unwrap();
    }
}
// Ah. The internal thread paniced, then the extenal thread found nothing.

// When the non-active thread panics or asserts, the main one quits unexpectedly.

//Ah. Console only shows basic test results and successful print msgs. Need to look at the console.


impl<T:Clone + Default + Send +  Eq + std::fmt::Debug + Sync + 'static> SuperstoreIn<T>{ // TODO2: Not sure why T needs to be sync to be sent into thread. Why not just send?
    fn write_data(&mut self, new_data: SuperstoreData<T>){
        let mut lukewarm = self.lukewarm.write().unwrap();
        for (i, item) in new_data.data.into_iter().enumerate(){
            if new_data.frame_offset + i < self.frame_offset{
                continue; // Ignore any frames of data about before where we start.
            }
            let relative_index = new_data.frame_offset + i - self.frame_offset;

            if lukewarm.len() > relative_index{
                lukewarm[relative_index] = item;
            }else{

                assert_eq!(relative_index, lukewarm.len(), "Tried to write item more than 1 past the end! Support could be added. Offset: {}", self.frame_offset);
                lukewarm.push(item);
            }
        }
    }
    pub fn start(mut self){
        thread::spawn(move||{
            loop {
                let new_data = match self.write_requests_rec.recv() {
                    Ok(data) => {
                        data
                    }
                    Err(error) => {
                        return; // If no more writes needed, can close thread.

                    }
                };
                self.write_data(new_data);
            }
        });
    }
}


mod tests{
    use super::*;

    #[test]
    fn test_superstore(){
//        let simed_frame = Arc::new(RwLock::new(-1));
//        let superstore = Arc::new(SuperstoreEx::start(0, simed_frame.clone()));
//
//
//        // TODO1: Add a thing to the test which infini-reads the last item until it's set.
//
//        let items_per_bunch = 10;
//        for i in 0..items_per_bunch{
//            superstore.test_set_simple(i, i);
//
//        }
//
//        thread::sleep(Duration::from_millis(500)); // dcwct Needed?
//        *simed_frame.write().unwrap() = 5;// dcwct Super crashes when this is 0.
//        // dcwct By writing waits in tests we're admitting it's a race.
//
//        for i in 0..items_per_bunch{
//            superstore.test_set_simple(i + items_per_bunch, i + items_per_bunch);
//        }
//
//        let expected_total = (items_per_bunch * 2 - 1) * items_per_bunch * 2 / 2;
//        let mut total = 0;
//        thread::sleep(Duration::from_millis(500)); // dcwct Needed?
//        for i in 0..(items_per_bunch * 2){
//            if i == 19{
//                let test = 2;
//            }
//            let result = superstore.get_clone(i);
//            assert!(result.is_some());
//            total += result.unwrap();
//        }
//        assert_eq!(total, expected_total);
    }
}





// TODO1: We also want to address blanks. If try to insert past end, should fill with blanks then set correct index.
