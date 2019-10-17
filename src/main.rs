
use std::env;


mod game;
mod network;
mod ecs;
mod players;
mod systems;
mod utils;


use crate::game::client::*;
use crate::game::server::*;





fn main() {
    println!("STARTING.");
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        match args[1].to_lowercase().as_ref() { // args[0] is the exe name.
            "client" => {
                client_main();
            }
            "server" => {
                server_main();
            }
            _ => {
                println!("Argument 1 wasn't 'server' or 'client'. Starting as client.");
                client_main();
            }
        }
    } else {
        println!("Server/client argument not specified. Starting as client.");
        client_main();
    }
}
