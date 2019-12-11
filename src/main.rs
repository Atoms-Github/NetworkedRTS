#![feature(core_intrinsics)]
#![feature(async_await)]
#![allow(dead_code)]
use std::env;


mod game;
mod network;
mod ecs;
mod players;
mod systems;
mod utils;


use crate::game::client::*;
use crate::game::server::*;
use ggez::input::keyboard::KeyCode;


fn main() {
    println!("STARTING.2");
    let mut args: Vec<String> = env::args().collect();

    args.reverse();
    let exe_name = args.pop();
    let launch_type = args.pop().expect("'client'/'server' argument not specified!");
    let ip = match args.pop() {
        Some(ip_str) => {
            ip_str
        }
        _ => {
            let default = "192.168.0.10".to_string();
            println!("Connection/hosting IP not specified! Using {}", default);
            default
        }
    }; // args.pop().or_else().expect("Connection/hosting IP not specified!");


    match launch_type.to_lowercase().as_ref() {
        "client" => {
            client_main(&ip);
        }
        "server" => {
            server_main(&ip);
        }
        _ => {
            println!("Argument 1 wasn't 'server' or 'client'. Starting as client.");
            client_main(&ip);
        }
    }


}
