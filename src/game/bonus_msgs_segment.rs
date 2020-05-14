use std::panic;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use serde::*;

use crate::game::synced_data_stream::*;
use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_structs::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BonusEvent{
    NewPlayer(PlayerID),
}
