use std::sync::{Arc, RwLock};
use crate::netcode::InputState;


pub type ArcRw<T> = Arc<RwLock<T>>;
pub type ThreadCloser = ();
pub type ServerEvents = Vec<InputState>;