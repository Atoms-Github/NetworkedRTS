#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(core_intrinsics)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_unsafe)] // TODO2: Investigate the need for this.
#![feature(drain_filter)]
#![allow(unused_attributes)]


use std::{env, thread};

use crate::client::client_mode::*;
use crate::server::server_mode::*;
use crate::common::types::*;
use std::str::FromStr;



pub mod client;
pub mod server;
pub mod common;

pub const DEBUG_MSGS_ALL: bool = false;
pub const DEBUG_MSGS_MAIN: bool = DEBUG_MSGS_ALL || false;
pub const DEBUG_MSGS_NET: bool = DEBUG_MSGS_ALL || true;
pub const WARN_MSGS: bool = DEBUG_MSGS_ALL || true; // TODO2 Could use warn/custom macros.
pub const DEBUG_MSGS_TIMERS: bool = DEBUG_MSGS_ALL || false;
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
            if !record.target().contains("poggy"){
                return std::io::Result::Ok(());
            }
            writeln!(buf, "{} [{}] {}", Local::now().format("%M:%S%.3f"), record.level(), record.args())
        }).filter(None, LevelFilter::Debug).init();
    log::info!("Starting!");


    let mut args: Vec<String> = env::args().collect();

    args.reverse();
    let _exe_name = args.pop();
    let launch_type = args.pop().expect("'client'/'server' argument not specified!");
    let ip = match args.pop() {
        Some(ip_str) => {
            ip_str
        }
        _ => {
            let default = "127.0.0.1:1414".to_string();
            log::info!("Connection/hosting IP not specified! Using {}", default);
            default
        }
    };


    let mut is_server = false;
    match launch_type.to_lowercase().as_ref() {
        "client" => {
            // Nothing.
        }
        "server" => {
            is_server = true;
        }
        _ => {
            log::debug!("Argument 1 wasn't 'server' or 'client'. Starting as client.");
        }
    }
    if is_server{
        server_main(ip);
    }else{
        let prefered_player_id = args.pop().map(|as_str|{
            i32::from_str(as_str.as_str()).ok()
        }).flatten().unwrap_or(0); // Conflict means auto-assign.
        ClientApp::go(String::from("A_toms"), ip, prefered_player_id);
    }
}