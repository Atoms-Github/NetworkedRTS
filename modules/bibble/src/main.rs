#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(core_intrinsics)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_unsafe)] // TODO2: Investigate the need for this.
#![feature(drain_filter)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(deprecated)] // TODO:

pub mod game;
pub mod compsys;
pub mod bibble;

pub use compsys::*;
pub use serde::{Deserializer, Deserialize, Serialize};
pub use netcode::*;
pub use becs::*;
pub use pcommon::*;
pub use netcode::common::net_game_state::StaticFrameData;


fn main() {
    println!("Hello, world!");
}
