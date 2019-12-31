use ggez::*;
use ggez::{ContextBuilder, event};
use futures::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::network::networking_structs::*;
use crate::network::networking_message_types::*;
use crate::players::inputs::*;
use ggez::event::{EventHandler, KeyMods};
use ggez::input::keyboard::KeyCode;
use crate::game::client_networking::connect_and_send_handshake;

use crate::systems::render::*;
use futures::future::lazy;

use crate::ecs::world::*;
use crate::ecs::system_macro::*;
use crate::network::*;

use crate::game::client_networking::*;

use futures::future::Future;

use std::time::{SystemTime};
use std::io::Write;
use bytes::Bytes;
use futures::sink::Sink;
use std::net::TcpStream;
use std::thread::Thread;
use crate::game::game_logic_layer;
use crate::game::timekeeping::KnownFrameInfo;
use crate::game::game_logic_layer::GameLogicLayer;
use std::sync::mpsc::{channel, Receiver, Sender};

use bus::Bus;

struct GameGraphicalLayer {
//    my_current_input_state: Arc<Mutex<InputState>>,
    render_head: Arc<Mutex<GameState>>,
    my_player_id: PlayerID,
    sender: Option<Sender<InputChange>>
}
/*
let (logic_layer, head_handle) =
            GameLogicLayer::new(true, known_frame_info, game_state);
        let channels = channel();
        thread::spawn(move ||{
            logic_layer.run_logic_loop();
        });\
*/
impl GameGraphicalLayer {
    pub fn new(head_render_handle: Arc<Mutex<GameState>>, my_player_id: PlayerID) -> GameGraphicalLayer{
        GameGraphicalLayer{
            render_head: head_render_handle,
            my_player_id,
            sender: None
        }
    }
    pub fn run_main(mut self) -> Receiver<InputChange>{
        let (sender,receiver) = channel();
        self.sender = Some(sender);

        let cb = ContextBuilder::new("Oh my literal pogger", "Atomsadiah")
            .window_setup(conf::WindowSetup::default().title("LiteralPoggyness"))
            .window_mode(conf::WindowMode::default().dimensions(500.0, 300.0)).add_resource_path(""); // TODO: Find what resource path.

        let (ctx, events_loop) = &mut cb.build().unwrap();

        thread::spawn(move ||{
            let mut meme = self;
            event::run(ctx, events_loop, &mut meme);
        });


        return receiver;
    }
}

impl EventHandler for GameGraphicalLayer {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
            // No logic currently :)
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);


        let mut pending = PendingEntities::new();
        let head_unlocked = Mutex::lock(&self.render_head).unwrap();
        secret_render_system(&head_unlocked.world, &mut pending,
                             &mut head_unlocked.storages.position_s,
                             &mut head_unlocked.storages.render_s,
                             &mut head_unlocked.storages.size_s,
                             ctx);

        head_unlocked.world.update_entities(&mut head_unlocked.storages, pending); // TODO ask Richard if this is really needed after calling render. (Unlikely)

        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        self.sender.unwrap().send(InputChange::KeyDownUp(keycode, true)).unwrap();

//        match keycode {
//            KeyCode::Escape => event::quit(ctx),
//            _ => (), // Do nothing
//        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.sender.unwrap().send(InputChange::KeyDownUp(keycode, false)).unwrap();
    }
}
