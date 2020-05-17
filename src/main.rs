#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(core_intrinsics)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)] // TODO2: Investigate the need for this.
#![feature(drain_filter)]

use std::env;

use crate::client::client::*;
use crate::server::server::*;

pub mod client;
pub mod server;
pub mod common;

pub const SEND_DEBUG_MSGS: bool = true;



fn main() {
    println!("STARTING2345.");
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

    let test_arg = args.pop().unwrap_or(String::from("0")).parse::<i64>().unwrap();

    match launch_type.to_lowercase().as_ref() {
        "client" => {
            client_main(ip, test_arg);
        }
        "server" => {
            server_main(ip);
        }
        _ => {
            println!("Argument 1 wasn't 'server' or 'client'. Starting as client.");
            client_main(ip, test_arg);
        }
    }
}