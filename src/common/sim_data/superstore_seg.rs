use std::net::{SocketAddr, TcpStream};
use std::ops::Add;
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, SystemTime};
use std::ops::Div;
use std::ops::Sub;
use serde::{Deserialize, Serialize};
use crate::common::network::external_msg::*;
use crate::common::types::*;
use crate::common::data::readvec::*;
use std::sync::{RwLock, Arc, RwLockWriteGuard, Mutex};
use std::collections::vec_deque::*;
use std::io::Seek;
use crate::client::input_handler_seg::*;
use crate::common::data::fake_read_vec::FakeReadVec;

type UsedReadVec<T> = FakeReadVec<T>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SuperstoreData<T> {
    pub data: Vec<T>,
    pub frame_offset: FrameIndex
}



struct SuperstoreIn<T:Clone + Default + Send +  Eq + std::fmt::Debug + Sync + 'static>{
    frame_offset: usize,
    hot_write: Box<VecDeque<T>>,
    hot_read: ArcRw<Box<VecDeque<T>>>,
    write_requests_rec: Receiver<SuperstoreData<T>>,
    cold: Arc<UsedReadVec<T>>,
    tail_simed_index: ArcRw<i32> // pointless_optimum: Can swap out for a racy thing.
}
#[derive()]
pub struct SuperstoreEx<T:Clone + Default + Send +  Eq + std::fmt::Debug + Sync + 'static>{
    frame_offset: usize,
    hot_read: ArcRw<Box<VecDeque<T>>>, // TODO2: Perhaps don't need box.
    cold: Arc<UsedReadVec<T>>,
    pub write_requests_sink: Mutex<Sender<SuperstoreData<T>>>, // pointless_optimum: Could setup a system where each thread has a local clone of this instead.
}


impl<T:Clone + Default + Send +  Eq + std::fmt::Debug + Sync + 'static> SuperstoreEx<T>{
    pub fn start(frame_offset: usize, tail_simed_index: ArcRw<i32>)-> Self{
        let (writes_sink, writes_rec) = channel();
        let hot_read = Arc::new(RwLock::new(Box::new(VecDeque::new())));

        let cold = Arc::new(UsedReadVec::new());

        SuperstoreIn{
            frame_offset,
            hot_write: Box::new(VecDeque::new()),
            hot_read: hot_read.clone(),
            write_requests_rec: writes_rec,
            cold: cold.clone(),
            tail_simed_index
        }.start();

        Self{
            frame_offset,
            hot_read,
            cold,
            write_requests_sink: Mutex::new(writes_sink)
        }
    }

    pub fn get_first_frame_index(&self) -> FrameIndex{
        return self.frame_offset;
    }
    pub fn get_clone(&self, abs_index: FrameIndex) -> Option<T>{ // optimum Shouldn't need to clone so much. Should be possible never cloning.
        assert!(abs_index >= self.frame_offset, format!("Tried to get data from before superstore start. TargetIndex: {}, FirstDataAt: {}", abs_index, self.frame_offset));
        let relative_index = abs_index - self.frame_offset;

        if self.cold.len() > relative_index{
            return self.cold.get(relative_index).cloned();
        }else{
            // Need to go into hot. We'll need to lock hot and re-check cold length as it might have changed.
            let hot_lock = self.hot_read.read().unwrap();
            if self.cold.len() > relative_index{
                return self.cold.get(relative_index).cloned();
            }else{
                let relative_to_hot = relative_index - self.cold.len();
                let test_hot_lock_len = hot_lock.len();
                if hot_lock.len() > relative_to_hot{
                    return hot_lock.get(relative_to_hot).cloned();
                }else{
                    return None;
                }
            }
        }
    }
    pub fn get_last_clone(&self) -> Option<T>{
        let hot_read = self.hot_read.read().unwrap();
        if hot_read.len() > 0{
            hot_read.get(hot_read.len() - 1).cloned()
        }else if self.cold.len() > 0{ // Can be false when client receives player list and all the superstores are inited.
            self.cold.get(self.cold.len() - 1).cloned()
        }else{
            return None;
        }
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
    fn write_data(&mut self, new_data: SuperstoreData<T>, validate_freezer: bool){
        if validate_freezer{
//            println!("Writing {} datas to {}", new_data.data.len(), new_data.frame_offset);
        }

//        Instead ignore any data before. assert!(new_data.frame_offset >= self.frame_offset, format!("Tried to write data earlier than superstore handles. TargetFrame: {} SuperstoresFirst: {}", new_data.frame_offset, self.frame_offset));


        let cold_length = self.cold.len();
        for (i, item) in new_data.data.into_iter().enumerate(){
            if new_data.frame_offset + i < self.frame_offset{
                continue; // Ignore any frames of data about before where we start.
            }
            let relative_index = new_data.frame_offset + i - self.frame_offset;
            if cold_length <= relative_index{ // If past cold bit.
                let hot_index = relative_index - cold_length;
                if hot_index < self.hot_write.len(){
                    self.hot_write[hot_index] = item;
                }else{
                    assert_eq!(hot_index, self.hot_write.len(), "Tried to write item more than 1 past the end! Support could be added."); // dcwct Should do something about blanks
                    self.hot_write.push_back(item);
                }
            }else{
                if validate_freezer{ // pointless_optimum
                    assert_eq!(self.cold.get(relative_index).unwrap(), &item, "Tried to write over cold data with different data!");
                }
            }
        }
    }
    fn cool/*I get to call a function "cool" :)*/(&mut self) -> usize{
        let simed_index = self.tail_simed_index.read().unwrap();

        let simed_index_relative = *simed_index - self.frame_offset as i32;

        let mut num_to_cool = (simed_index_relative + 1 - self.cold.len() as i32).max(0);

//        assert!(num_to_cool >= 0, format!("Cooling negative amounts: {} simed_index: {} frame offset: {}", num_to_cool, *simed_index, self.frame_offset)); // dcwct. Hmm.

        for i in 0..num_to_cool{
            assert!(self.hot_write.len() > 0, format!("Simed frame was ahead of the cooling wave! How did we sim tail without data? SimedFrame: {}", *simed_index));
            let new_item_to_freeze = self.hot_write.pop_front().unwrap();
            self.cold.push(new_item_to_freeze);
        }


        return num_to_cool as usize;


    }
    fn pop_hot(&mut self, num_to_pop: usize){
        for i in 0..num_to_pop{
            self.hot_write.pop_front(); // pointless_optimum
        }
    }
    // TODO1: Replace recv loops with channel.iter()
    pub fn start(mut self){
        thread::spawn(move||{
            let hot_read_arc = self.hot_read.clone();
            loop{
                let new_data = match self.write_requests_rec.recv(){
                    Ok(data) => {
                        data
                    }
                    Err(error) => {
                        return; // If no more writes needed, can close thread.
                    }
                };
                // We want to:
                // - Write to local. (With cold validation)
                // - Lock pub.
                // - Cool using local. (After lock so gets to read only aren't wrong)
                // - Swap pub and local.
                // - Unlock pub.
                // - Pop local same as other local.
                // - Write same data to local again. (No need for validation)


                // We want to:
                // - Write to local. (With cold validation)
                self.write_data(new_data.clone(), true);
                // - Lock pub.
                let mut hot_read_handle = hot_read_arc.write().unwrap();
                // - Cool using local. (After lock so gets to read only aren't wrong)
                let items_cooled = self.cool();
                // - Swap pub and local.


                std::mem::swap(&mut self.hot_write, &mut hot_read_handle); // TODO1: Should just need to swap boxes. This might be copying data.

                // - Unlock pub.
                std::mem::drop(hot_read_handle);

                // - Pop local same as other local.
                self.pop_hot(items_cooled);
                // - Write same data to local again. (No need for validation)
                self.write_data(new_data.clone() /* dcwct No clone*/, false);

            }
        });
    }
}


mod tests{
    use super::*;

    #[test]
    fn test_superstore(){
        let simed_frame = Arc::new(RwLock::new(-1));
        let superstore = Arc::new(SuperstoreEx::start(0, simed_frame.clone()));


        // TODO1: Add a thing to the test which infini-reads the last item until it's set.

        let items_per_bunch = 10;
        for i in 0..items_per_bunch{
            superstore.test_set_simple(i, i);

        }

        thread::sleep(Duration::from_millis(500)); // dcwct Needed?
        *simed_frame.write().unwrap() = 5;// dcwct Super crashes when this is 0.
        // dcwct By writing waits in tests we're admitting it's a race.

        for i in 0..items_per_bunch{
            superstore.test_set_simple(i + items_per_bunch, i + items_per_bunch);
        }

        let expected_total = (items_per_bunch * 2 - 1) * items_per_bunch * 2 / 2;
        let mut total = 0;
        thread::sleep(Duration::from_millis(500)); // dcwct Needed?
        for i in 0..(items_per_bunch * 2){
            if i == 19{
                let test = 2;
            }
            let result = superstore.get_clone(i);
            assert!(result.is_some());
            total += result.unwrap();
        }
        assert_eq!(total, expected_total);
    }
}





// TODO1: We also want to address blanks. If try to insert past end, should fill with blanks then set correct index.





// When do we step forward?
// Usually nothing will be arriving near end of hot.
// Once we get data for latest is a perfectly good time to move, right?
// Use vec_deque. Only time we need to adjust is with the ends.

// Don't cool things when hot is full. Instead grow hot, and only cool when tail ready.









// When you say get, it's easy if things are stationary.
// Things which might happen:
// New write:
// If it is a new frame:
//
// Ohza noza. Superstore might need to wait and not freeze inputs.
// Only freeze when have all the players? No. This isn't this level.


// We're going to keep exactly 20 spots open for writing.
// We don't have an option for adding items in the future.


// The 20 spots represent the 20 on the end.
// There's only 1 spot where things can be added onto the end.

// If we don't have any precautions, SuperstoreEx won't know where to look.
// If the order happens arc then push, then could get wrong data.
// If the order happens push then arc, then could get wrong data.

// Need to lock getting when arc is moving forward - which is fair :)
// Or is it fair?
// The idea was we wouldn't ever need to lock.

// Can you lock the arc, advance the push, then unlock the arc?