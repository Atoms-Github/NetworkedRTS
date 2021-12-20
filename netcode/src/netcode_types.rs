use std::sync::{Arc, RwLock};
use crate::netcode::common::confirmed_data::ServerEvent;


pub type ArcRw<T> = Arc<RwLock<T>>;
pub type ThreadCloser = ();
pub type ServerEvents = Vec<ServerEvent>;