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
use std::sync::{RwLock, Arc, RwLockWriteGuard};
use std::collections::vec_deque::*;
use std::io::Seek;
use crate::client::input_handler_seg::*;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SuperstoreData<T> {
    pub data: Vec<T>,
    pub frame_offset: FrameIndex
}



pub struct SuperstoreIn<T:Default + Send + Eq + 'static>{
    frame_offset: usize,
    hot_write: Box<VecDeque<T>>,
    hot_read: ArcRw<Box<VecDeque<T>>>,
    write_requests_rec: Receiver<SuperstoreData<T>>,
    cold: ReadVec<T>,
    tail_simed_index: ArcRw<FrameIndex> // pointless_optimum: Can swap out for a racy thing.
}
#[derive(Clone)]
pub struct SuperstoreEx<T:Default + Send + Eq + 'static>{
    frame_offset: usize,
    hot_read: ArcRw<Box<VecDeque<T>>>, // TODO2: Perhaps don't need box.
    cold: ReadVec<T>
}


impl<T:Default + Send + Eq + 'static> SuperstoreEx<T>{
    pub fn start(frame_offset: usize, tail_simed_index: ArcRw<FrameIndex>)-> (Self, Sender<SuperstoreData<T>>){
        let (writes_sink, writes_rec) = channel();
        let hot_read = Arc::new(RwLock::new(Box::new([T::default(); 20])));

        let cold = Arc::new(ReadVec::new());

        SuperstoreIn{
            frame_offset,
            hot_write: Box::new([T::default(); 20]),
            hot_read: hot_read.clone(),
            write_requests_rec: writes_rec,
            cold: cold.clone(),
            tail_simed_index
        }.start();

        (Self{
            frame_offset,
            hot_read,
            cold
        }, writes_sink)
    }

    pub fn get(&self, abs_index: FrameIndex) -> Option<&T>{
        let relative_index = abs_index - self.frame_offset;

        if self.cold.len() > relative_index{
            return self.cold.get(relative_index);
        }else{
            // Need to go into hot. We'll need to lock hot and re-check cold length as it might have changed.
            let hot_lock = self.hot_read.read().unwrap();
            if self.cold.len() > relative_index{
                return self.cold.get(relative_index);
            }else{
                let relative_to_hot = relative_index - self.cold.len();
                if hot_lock.len() > relative_to_hot{
                    return hot_lock[relative_to_hot];
                }else{
                    return None;
                }
            }
        }
    }
    pub fn get_last(&self) -> Option<&T>{
        let hot_read = self.hot_read.read().unwrap();
        if hot_read.len() > 0{
            hot_read.last()
        }else{
            self.cold.get(self.cold.len() - 1)
        }
    }
    pub fn push_simple(&self, data: T, frame_index: FrameIndex){
        self.write_requests_sink.send(SuperstoreData{
            data: vec![data],
            frame_offset: frame_index
        });
    }
}

impl<T:Default + Send + Eq + 'static> SuperstoreIn<T>{ // TODO2: Not sure why T needs to be sync to be sent into thread. Why not just send?
    fn write_data(&mut self, new_data: SuperstoreData<T>, validate_freezer: bool){
        let relative_index = new_data.frame_offset - self.frame_offset;

        let cold_length = self.cold.len();
        for item in new_data.data{
            if cold_length <= relative_index{ // If past cold bit.
                let hot_index = relative_index - cold_length;
                if hot_index < self.hot_write.len(){
                    self.hot_write[hot_index] = item;
                }else{
                    self.hot_write.push_back(item);
                }
            }else{
                if validate_freezer{ // pointless_optimum
                    assert!(self.cold.get(relative_index).unwrap().eq(item), "Tried to write over cold data with different data!");
                }
            }
        }
    }
    fn cool/*I get to call a function "cool" :)*/(&mut self) -> usize{
        let simed_index = self.tail_simed_index.read().unwrap();
        let mut num_to_cool = *simed_index + 1 - self.cold.len();

        for i in 0..num_to_cool{
            let new_item_to_freeze = self.hot_write.pop_front().unwrap_or_default();
            self.cold.push(new_item_to_freeze);
        }


        return num_to_cool;


    }
    fn pop_hot/*I get to call a function "cool" :)*/(&mut self, num_to_pop: usize){
        for i in 0..num_to_pop{
            self.hot_write.pop_front(); // pointless_optimum
        }
    }
    pub fn start(mut self){
        thread::spawn(move||{
            loop{
                let new_data = self.write_requests_rec.recv().unwrap();

                // We want to:
                // - Write to local. (With cold validation)
                // - Lock pub.
                // - Cool using local. (After lock so gets to read only aren't wrong)
                // - Swap pub and local.
                // - Unlock pub.
                // - Write same data to local again. (No need for validation)
                // - Pop local same as other local.


                // We want to:
                // - Write to local. (With cold validation)
                self.write_data(new_data.clone(), true);
                // - Lock pub.
                let mut hot_read_handle = self.hot_read.write().unwrap();
                // - Cool using local. (After lock so gets to read only aren't wrong)
                let items_cooled = self.cool();
                // - Swap pub and local.
                let swap_temp = *hot_read_handle; // Save pub.
                *hot_read_handle = self.hot_write; // Set pub to local.
                self.hot_write = swap_temp;
                // - Unlock pub.
                std::mem::drop(hot_read_handle);
                // - Write same data to local again. (No need for validation)
                self.write_data(new_data, false);
                // - Pop local same as other local.
                self.pop_hot(items_cooled);
            }
        });
    }
}

#[test]
fn test_superstore(){
    let simed_frame = Arc::new(RwLock::new(0));
    let superstore = Arc::new(SuperstoreEx::start(0, simed_frame.clone()));

    let mut threads = vec![];


    let thread_count = 5;
    let push_per_thread = 10;
    for thread_index in 0..thread_count{
        let capture = superstore.clone();
        let thread = thread::spawn(move ||{
            for i in 0..push_per_thread{
                capture.push_simple(thread_index, i + thread_index * thread_count);
            }
        });
        threads.push(thread);
    }

    *simed_frame.write().unwrap() = 30;

    for thread_index in 0..thread_count{
        let capture = superstore.clone();
        let thread = thread::spawn(move ||{
            for i in 0..push_per_thread{
                capture.push_simple(thread_index, i + thread_index * thread_count);
            }
        });
        threads.push(thread);
    }

    for thread in threads{
        crate::assert_result_ok(thread.join());
    }

    // TODO1: Implement test that checks order too.
}




// TODO1: We also want to address blanks. If try to insert past end, should fill with blanks then set correct index.





// When do we step forward?
// Usually nothing will be arriving near end of hot.
// Once we get data for latest is a perfectly good time to move, right?
// Use vec_deque. Only time we need to adjust is with the ends.

// Don't cool things when hot is full. Instead grow hot, and only cool when tail ready.





// // // // // //






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










