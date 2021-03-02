use std::sync::{Arc, RwLock};
use crate::netcode::InputState;
use crate::netcode::common::sim_data::sim_data_storage::ServerEvent;


pub type ArcRw<T> = Arc<RwLock<T>>;
pub type ThreadCloser = ();
pub type ServerEvents = Vec<ServerEvent>;