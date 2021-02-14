use std::sync::{Arc, RwLock};


pub type ArcRw<T> = Arc<RwLock<T>>;
pub type ThreadCloser = ();