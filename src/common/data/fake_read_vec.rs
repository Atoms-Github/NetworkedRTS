//use std::ptr::null;
//use std::sync::{Mutex, Arc, RwLock};
//use std::thread;
//use crate::common::types::ArcRw;
//
//#[derive()]
//pub struct FakeReadVec<T>{
//    data: ArcRw<Vec<T>>
//}
//unsafe impl<T> Send for FakeReadVec<T> {}
//unsafe impl<T> Sync for FakeReadVec<T> {}
//
//impl<T> FakeReadVec<T>{
//    pub fn new() -> FakeReadVec<T>{
//        FakeReadVec{
//            data: Arc::new(RwLock::new(vec![]))
//        }
//    }
//    pub fn push(&self, new_item: T){
//        let mut lock = self.data.write().unwrap();
//        lock.push(new_item);
//    }
//    pub fn get(&self, index: usize) -> Option<&T>{
//        let lock = self.data.read().unwrap();
//        match lock.get(index){
//            Some(item) => {
//
//            }
//        }
//    }
//    pub fn len(&self) -> usize {
//        let lock = self.data.read().unwrap();
//        return lock.len();
//    }
//}
