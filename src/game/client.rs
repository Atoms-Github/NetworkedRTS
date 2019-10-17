use ggez::{ContextBuilder, event};
use futures::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};

use crate::network::networking_structs::*;
use crate::players::inputs::*;

struct ClientMainState {
    game_state_head: GameState,
    game_state_tail: GameState,


//    messages_to_process: Arc<Mutex<Vec<message_types::NetMessageType>>>,
}




pub fn client_main(){
    println!("Starting as client.");


    let cb = ContextBuilder::new("Of my literal pogger", "Atomsadiah")
        .window_setup(conf::WindowSetup::default().title("LiteralPoggyness"))
        .window_mode(conf::WindowMode::default().dimensions(500.0, 300.0));

    let (ctx, events_loop) = &mut cb.build()?;


    let game = &mut ClientMainState::new(ctx)?;

    let local_players_mutex_cloned = game.local_players.clone();
    let online_players_mutex_cloned = game.messages_to_process.clone();

    let (stdin_tx, stdin_rx) = mpsc::channel(0);
    let thread_ip_sending = target_ip.clone();
    thread::spawn(|| net_input_transfers::keep_sending_inputs(thread_ip_sending, stdin_tx, local_players_mutex_cloned));

    let thread_ip_hosting = hosting_ip.clone();
    thread::spawn(move || net_input_transfers::keep_receiving_inputs(thread_ip_hosting, stdin_rx, online_players_mutex_cloned));

    event::run(ctx, events_loop, game)


}