

// We want to allow something to be 'writing' on the end of the vec as things are reading everywhere else.
// This means writing can't cause things to move around.
// So we need to use fixed length arrays instead of vecs as vecs move as they resize.

// We don't want to allocate too much memory before it's used, so we're going to use a two tiered system.
// This means our max capacity is the square of what we initially allocate.

const BLOCK_SIZE: usize = 1000;

struct ReadBlock<T>{
    data: [T; BLOCK_SIZE]
}


pub struct ReadVec<T>{
    data: [*const ReadBlock<T>; BLOCK_SIZE],
    size: usize
}

impl<T> ReadVec<T>{
    pub fn new(max_store: usize, block_size: usize) -> ReadVec<T>{
        ReadVec{
            data: vec![],
            size: 0
        }
    }
}

