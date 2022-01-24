#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(core_intrinsics)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_unsafe)] // TODO2: Investigate the need for this.
#![feature(drain_filter)]
#![allow(unused_attributes)]
#![allow(deprecated)] // TODO

use nalgebra::U2;

pub use pub_types::*;
pub use args::simple_game;

use crate::common::input_state::InputState;
use crate::common::net_game_state::GameState;

mod server;
mod client;
pub mod common;
mod utils;

mod pub_types;
pub mod args;

pub fn server_main<T : 'static + GameState>(hosting_ip: String){
    server::server_mode::server_main::<T>(hosting_ip);
}
pub fn client_main<T : 'static + GameState>(player_name: String, connection_ip: String, preferred_port: i32){
    let lower_name = player_name.to_lowercase();
    let my_color : Shade = match &lower_name[..]{
        "atoms" => {
            Shade(0.258, 0.529, 0.960)
        }
        "quicktoast" => {
            Shade(0.156, 0.823, 0.372)
        }
        "oberdiah" => {
            Shade(0.882, 0.035, 0.101)
        }
        "legend" => {
            Shade(1.0, 1.0, 1.0) // White. ?
        }
        "numcake" => {
            Shade(0.956, 0.517, 0.121)
        }
        "shotekri" => {
            Shade(0.956, 0.145, 0.596)
        }
        "connorhs" => {
            Shade(0.917, 0.956, 0.145) // Yellow. ?
        }
        "lain" => {
            Shade(0.145, 0.956, 0.607) // Turquoise.
        }
        "guest2" => {
            Shade(0.537, 0.145, 0.956) // Purple.
        }
        "guest3" => {
            Shade(0.301, 0.301, 0.301) // Dark grey.
        }
        "guest4" => {
            Shade(0.588, 0.588, 0.588) // Light grey.
        }
        _ => {
            Shade(0.0,0.0,0.0)
        }
    };
    client::client_mode::Client::<T>::go(player_name, my_color, connection_ip, preferred_port);
}


