#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(core_intrinsics)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_unsafe)] // TODO2: Investigate the need for this.
#![feature(drain_filter)]
#![allow(unused_attributes)]

#[macro_use]
extern crate mopa;


use std::{env, thread};


use crate::pub_types::*;
use crate::args::*;
use std::str::FromStr;



pub mod netcode;
pub mod bibble;
pub mod rts;
pub mod pub_types;
pub mod ecs;
pub mod utils;
pub mod args;


pub const DEBUG_MSGS_ALL: bool = false;
pub const DEBUG_MSGS_MAIN: bool = DEBUG_MSGS_ALL || false;
pub const DEBUG_MSGS_NET: bool = DEBUG_MSGS_ALL || true;
// pub const DEBUG_MSGS_NET: bool = DEBUG_MSGS_ALL || false;
pub const WARN_MSGS: bool = DEBUG_MSGS_ALL || true; // TODO2 Could use warn/custom macros.
pub const DEBUG_MSGS_TIMERS: bool = DEBUG_MSGS_ALL || true;
pub const DEBUG_MSGS_PROCESS: bool = DEBUG_MSGS_ALL || true;

use crossbeam_channel::{unbounded, Select};


use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::time::Duration;



fn main() {
    Builder::new()
        .format(|buf, record| {
            if record.target().contains("poggy"){
                return writeln!(buf, "{} [{}] {}", Local::now().format("%M:%S%.3f"), record.level(), record.args());
            }
            return std::io::Result::Ok(());
        }).filter(None, LevelFilter::Info).init();
    log::info!("Starting!");

    let args = crate::args::Args::gather();
    let address = args.ip + ":1414";


    match args.launch_type{
        LaunchType::CLIENT => {
            crate::netcode::client_main(args.player_name.unwrap(), address, 0);
        }
        LaunchType::SERVER => {
            crate::netcode::server_main(address);
        }
    }
}


// fn test1(){
//     crate::server::net_hub_back_not_seg::hub_back_test::print_listened();
// }
// fn test2(){
//     crate::client::connect_net_seg::connect_tests::crash_on_connect();
// }
// fn test3(){
//     crate::client::connect_net_seg::connect_tests::wait_on_connect();
// }
// fn test4(){
//     crate::client::connect_net_seg::connect_tests::crash_on_connect();
// }


// knownissue client can get in 'requesting things' stuck phase.
// If it requests things far behind the input list's range, when it learns of this data, it won't save it.
// Fixable by:
// a. Make the input list a large hashmap (slow).
// b. Making the list all option<T>, (slow + whack).
// c. Something super whack (whack + whack).