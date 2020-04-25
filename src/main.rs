#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(core_intrinsics)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)] // TODO2: Investigate the need for this.
#![feature(drain_filter)]

use std::env;

use crate::game::client::*;
use crate::game::server::*;

mod game;
mod network;
mod ecs;
mod players;
mod gameplay;
mod utils;

pub const SEND_DEBUG_MSGS: bool = true;



fn main() {
    println!("STARTING234.");
    let mut args: Vec<String> = env::args().collect();

    args.reverse();
    let _exe_name = args.pop();
    let launch_type = args.pop().expect("'client'/'server' argument not specified!");
    let ip = match args.pop() {
        Some(ip_str) => {
            ip_str
        }
        _ => {
            let default = "25.84.114.249:5535".to_string();
            println!("Connection/hosting IP not specified! Using {}", default);
            default
        }
    }; // args.pop().or_else().expect("Connection/hosting IP not specified!");


    match launch_type.to_lowercase().as_ref() {
        "client" => {
            client_main(ip);
        }
        "server" => {
            server_main(ip);
        }
        _ => {
            println!("Argument 1 wasn't 'server' or 'client'. Starting as client.");
            client_main(ip);
        }
    }
}