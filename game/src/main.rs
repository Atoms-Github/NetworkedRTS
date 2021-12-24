#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(core_intrinsics)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_unsafe)] // TODO2: Investigate the need for this.
#![feature(drain_filter)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(deprecated)]

#[macro_use]
extern crate mopa;


use std::{env, thread};


use crate::pub_types::*;
use crate::args::*;
use std::str::FromStr;



pub mod bibble;
pub mod rts;
pub mod pub_types;
pub mod ecs;
pub mod utils;
pub mod unsafe_utils;
pub mod args;


pub const DEBUG_MSGS_ALL: bool = false;
pub const DEBUG_MSGS_ITS_LAGGING: bool = false;
pub const DEBUG_MSGS_MAIN: bool = DEBUG_MSGS_ALL || false;
pub const DEBUG_MSGS_NET: bool = DEBUG_MSGS_ALL || true;
// pub const DEBUG_MSGS_NET: bool = DEBUG_MSGS_ALL || false;
pub const WARN_MSGS: bool = DEBUG_MSGS_ALL || true; // TODO2 Could use warn/custom macros.
pub const DEBUG_MSGS_TIMERS: bool = DEBUG_MSGS_ALL || false;
pub const DEBUG_MSGS_PROCESS: bool = DEBUG_MSGS_ALL || true;

use crossbeam_channel::{unbounded, Select};


use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use std::time::Duration;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use crate::rts::GameStateJigsaw;


fn main() {

    let args = crate::args::Args::gather();
    let address = args.ip + ":1616";
    Builder::new()
        .format(|buf, record| {
            if record.target().contains("poggy"){
                return writeln!(buf, "{} [{}] {}", Local::now().format("%M:%S%.3f"), record.level(), record.args());
            }
            return std::io::Result::Ok(());
        }).filter(None, LevelFilter::Info).init();

    // let mut args_str: Vec<String> = env::args().collect();
    // let logfile = FileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
    //     .build(format!("log/output{}.log", args_str[1])).unwrap();
    //
    // let config = Config::builder()
    //     .appender(Appender::builder().build("logfile", Box::new(logfile)))
    //     .build(Root::builder()
    //         .appender("logfile")
    //         .build(LevelFilter::Warn)).unwrap();
    //
    // log4rs::init_config(config).unwrap();


    log::info!("Starting!");




    match args.launch_type{
        LaunchType::CLIENT => {
            netcode::client_main::<GameStateJigsaw>(args.player_name.unwrap(), address, 0);
        }
        LaunchType::SERVER => {
            netcode::server_main::<GameStateJigsaw>(address);
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