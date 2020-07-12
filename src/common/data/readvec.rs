use std::ptr::null;
use std::sync::{Mutex, Arc};
use std::thread;


// - - -
// Aim: To make a vector which always allows reading, and allows up to one writer. This way readers never have to wait and can take as long as they like.
// - - -


// We want to allow something to be 'writing' on the end of the vec as things are reading everywhere else.
// This means writing can't cause things to move around.
// So we need to use fixed length arrays instead of vecs as vecs move as they resize.

// We don't want to allocate too much memory before it's used, so we're going to use a two tiered system.
// This means our max capacity is around the square of the initial allocation.

const BLOCK_SIZE: usize = 1; // Number of structs created at once.
const BLOCK_COUNT: usize = 5; // Number of pointers to struct blocks.
const MAX_CAPACITY: usize = BLOCK_SIZE * BLOCK_COUNT;


struct ReadBlock<T:Copy>{
    items: [T; BLOCK_SIZE],
    items_populated: usize
}

unsafe impl<T:Copy> Send for ReadBlock<T> {}
unsafe impl<T:Copy> Sync for ReadBlock<T> {}



pub struct ReadVec<T:Copy>{
    blocks_pointers: [*const ReadBlock<T>; BLOCK_COUNT],
    blocks_vec: Vec<Box<ReadBlock<T>>>, // This just stores a bunch of T and deletes them at the right time.
    blocks_full: usize,
    write_lock: Mutex<()>
}
unsafe impl<T:Copy> Send for ReadVec<T> {}
unsafe impl<T:Copy> Sync for ReadVec<T> {}

impl<T:Copy> ReadVec<T>{
    pub fn new() -> ReadVec<T>{
        ReadVec{
            blocks_pointers: [null(); BLOCK_COUNT],
            blocks_vec: vec![],
            blocks_full: 0,
            write_lock: Mutex::new(())
        }
    }
    fn hack_make_mut(&self) -> &mut Self{
        unsafe{
            return &mut *(self as *const Self as *mut Self);
        }
    }
    pub fn push(&self, new_item: T){
        let mut lock = self.write_lock.lock().unwrap(); // Lock other writes until write finished.

        let self_mut = self.hack_make_mut();

        if self.blocks_full == self.blocks_vec.len(){ // If we need to make a new block because all blocks are full.
            if !(self.blocks_full < BLOCK_COUNT){ // TODO simple
                std::mem::drop(lock);
                assert!(false, "Capacity exceeded!"); // TODO1: Can merge.
            }

            let new_block = ReadBlock{
                items: [new_item; BLOCK_SIZE],
                items_populated: 0
            };
            let boxed = Box::new(new_block);
            let block_location = &(*boxed) as *const ReadBlock<T>; // Get where the box points to.
            self_mut.blocks_vec.push(boxed);

            self_mut.blocks_pointers[self.blocks_full] = block_location;
        }

        let mut non_full_block= self_mut.blocks_vec.last_mut().unwrap(); // Guarenteed to not be full.
        non_full_block.items[non_full_block.items_populated] = new_item;

        // TODO1: A read can come in at any time. Need to make sure every order of things keeps things ok.
        non_full_block.items_populated += 1;
        if non_full_block.items_populated == BLOCK_SIZE{
            self_mut.blocks_full += 1;
        }
    }
    pub fn get(&self, index: usize) -> Option<&T>{
        let inblock_index = index % BLOCK_SIZE;
        let outblock_index = (index - inblock_index) / BLOCK_SIZE;

        if self.blocks_pointers[outblock_index] != null() {
            unsafe{
                let block = &*self.blocks_pointers[outblock_index];
                if inblock_index < block.items_populated{
                    return Some(&block.items[inblock_index]);
                }
            }
        }
        return None;
    }
    pub fn len(&self) -> usize {
        let items_in_non_full = if self.blocks_full == BLOCK_COUNT || self.blocks_pointers[self.blocks_full] == null(){
            0
        }else{
            unsafe{
                (*self.blocks_pointers[self.blocks_full]).items_populated
            }
        };
        return self.blocks_full * BLOCK_SIZE + items_in_non_full;
    }
}



fn assert_result_ok(r: thread::Result<()>) {
    let ok = r.is_ok();
    match r {
        Ok(r) => {},
        Err(e) => {
            if let Some(e) = e.downcast_ref::<&'static str>() {
                println!("Got an error: {}", e);
            } else {
                println!("Got an unknown error: {:?}", e);
            }
        }
    }
    assert!(ok, "Thread crashed. See print for msg.");
}



#[test]
fn test_loads_of_write_fails(){
    let read_vec = Arc::new(ReadVec::<usize>::new());

    let mut write_threads = vec![];

    let thread_count = 100;
    let items_per_thread = 2;

    // Is it when another thread asserts it poisons the mutex?

    for index in 0..thread_count{
        let capture_read = read_vec.clone();
        let thread = thread::spawn(move ||{

            let mut actions = 0;
            while actions < items_per_thread{
                actions += 1;
                capture_read.push(3);
            }
        });
        write_threads.push(thread);
    }

    for thread in write_threads{
        assert_result_ok(thread.join());
    }
    assert!(*read_vec.get(BLOCK_SIZE + 1).unwrap() == 3);

    assert!(read_vec.len() == thread_count * items_per_thread);
}


#[test]
fn test_big_mess_of_everything(){
    let read_vec = Arc::new(ReadVec::<usize>::new());

    read_vec.push(2);


    let thread_count = 200;
    let items_per_thread = 200;

    let mut write_threads = vec![];
    let mut read_threads = vec![];
    for index in 0..thread_count{
        let capture_read = read_vec.clone();
        let thread = thread::spawn(move ||{
            let mut actions = 0;
            while actions < items_per_thread{
                actions += 1;
                capture_read.push(3);
            }
        });
        write_threads.push(thread);
    }

    for index in 0..thread_count{
        let capture_read = read_vec.clone();
        let thread = thread::spawn(move ||{
            let mut actions = 0;
            while actions < items_per_thread{
                actions += 1;
                assert!(*capture_read.get(0).unwrap() == 2);
            }
        });
        read_threads.push(thread);
    }

    for thread in write_threads{
        assert_result_ok(thread.join());
    }
    for thread in read_threads{
        assert_result_ok(thread.join());
    }
    assert!(*read_vec.get(BLOCK_SIZE + 1).unwrap() == 3);


    assert!(read_vec.len() == thread_count * items_per_thread + 1);

    // TODO1: Add up results and test.
}


const BASIC_PUSH_GET_COUNT: usize = BLOCK_SIZE * 2 + 5;

#[test]
fn test_basic_write_read(){
    let small_vec = ReadVec::<usize>::new();
    assert!(small_vec.len() == 0);

    for test_index in 0..BASIC_PUSH_GET_COUNT{
        small_vec.push(test_index);
        assert!(*small_vec.get(test_index).unwrap() == test_index);
    }

    assert!(small_vec.len() == BASIC_PUSH_GET_COUNT);
}


#[test]
fn test_write_while_read(){
    let read_vec = Arc::new(ReadVec::<usize>::new());
    read_vec.push(6);

    let capture_read = read_vec.clone();
    let read_thread = thread::spawn(move ||{
        let mut reads = 0;
        while reads < 100_000{
            reads += 1;
            assert!(*capture_read.get(0).unwrap() == 6);
        }
    });

    let capture_write = read_vec.clone();
    let write_thread = thread::spawn(move ||{
        let mut writes = 0;

        while writes < MAX_CAPACITY - 1{
            writes += 1;
            capture_write.push(100);
        }
    });

    assert_result_ok(read_thread.join());
    assert_result_ok(write_thread.join());

    assert!(read_vec.len() == MAX_CAPACITY);
}


#[test]
fn test_loads_of_read(){
    let read_vec = Arc::new(ReadVec::<usize>::new());
    read_vec.push(6);

    let mut read_threads = vec![];

    for index in 0..200{
        let capture_read = read_vec.clone();
        let read_thread = thread::spawn(move ||{
            let mut reads = 0;
            while reads < 10_000{
                reads += 1;
                assert!(*capture_read.get(0).unwrap() == 6);
            }
        });
        read_threads.push(read_thread);
    }

    for thread in read_threads{
        assert_result_ok(thread.join());
    }

    assert!(read_vec.len() == 1);
}


