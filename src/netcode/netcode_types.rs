use std::sync::{Arc, RwLock};

type FrameIndex = usize;
type ArcRw<T> = Arc<RwLock<T>>;
type HashType = u64;
pub type ThreadCloser = ();