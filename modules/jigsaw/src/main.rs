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


pub mod compsys;
pub mod archetypes;
pub mod jigsaw_game_state;

pub use compsys::*;
pub use serde::{Deserializer, Deserialize, Serialize};
pub use netcode::*;
pub use becs::*;
pub use pcommon::*;
pub use netcode::common::net_game_state::StaticFrameData;


use ggez::input::gamepad::gilrs::ev::filter::FilterFn;
use env_logger::Builder;

use std::{env, thread};
use std::io::Write;
use std::str::FromStr;
use std::time::Duration;

use chrono::Local;
use log::LevelFilter;

fn main() {
    Builder::new()
        .format(|buf, record| {
            if record.target().contains("poggy"){
                return writeln!(buf, "{} [{}] {}", Local::now().format("%M:%S%.3f"), record.level(), record.args());
            }
            return std::io::Result::Ok(());
        }).filter(None, LevelFilter::Info).init();

    netcode::simple_game::<GameStateJigsaw>()
}
