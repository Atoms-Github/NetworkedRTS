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

pub(crate) use serde::*;
pub(crate) use pcommon::ResourceBlock;
pub(crate) use data_types::*;
pub mod races;
pub use data_types::*;
use races::*;

mod data_types;

impl GameData{
    pub fn gen_game_data() -> Self{
        let mut game_data = GameData{
            units: Default::default(),
            races: Default::default(),
            abilities: Default::default()
        };
        races::gather_races(&mut game_data);


        return game_data;
    }
}
