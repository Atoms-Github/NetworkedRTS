use crossbeam_channel::*;
use std::thread;

use ggez::*;
use ggez::{ContextBuilder, event};
use ggez::event::{EventHandler, KeyMods, MouseButton};
use ggez::input::keyboard::KeyCode;

use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use crate::netcode::*;
use crate::netcode::common::sim_data::input_state::*;
use std::time::SystemTime;
use ggez::graphics::Text;
use std::collections::BTreeMap;
use nalgebra::Point2;
use crate::netcode::common::sim_data::net_game_state::{NetPlayerProperty, NetGameState};
use std::sync::Arc;
use crate::rts::compsys::RenderResources;
use crate::rts::GameState;

pub struct GraphicalIn {
    render_head_rec: Receiver<NetGameState>,
    my_player_id: PlayerID,
    input_sink: Sender<InputChange>,
    texts: BTreeMap<&'static str, Text>,
    resources: ResourcesPtr
}
pub struct GraphicalEx {
    pub input_rec: Receiver<InputChange>,
}
impl GraphicalEx{
    pub fn start(head_render_handle: Receiver<NetGameState>, my_player_id: PlayerID) -> GraphicalEx{
        let (input_sink, input_rec) = unbounded();


        let mut texts = BTreeMap::new();
        let text = Text::new("Hello, World!");
        // Store the text in `App`s map, for drawing in main loop.
        texts.insert("0_hello", text);


        GraphicalIn{
            render_head_rec: head_render_handle,
            my_player_id,
            input_sink,
            texts,
            resources: GameState::gen_resources()
        }.start();

        GraphicalEx{
            input_rec
        }
    }
}
impl GraphicalIn {
    pub fn start(mut self){
        let cb = ContextBuilder::new("Oh my literal pogger", "Atomsadiah")
            .window_setup(conf::WindowSetup::default().title("LiteralPoggyness"))
            .window_mode(conf::WindowMode::default().dimensions(960.0, 540.0)).add_resource_path(""); // TODO3: Find what resource path.




        thread::spawn(move ||{
            let (ctx, events_loop) = &mut cb.build().unwrap();

            let mut meme = self;
            event::run(ctx, events_loop, &mut meme).unwrap();

            log::info!("Shutting down.");
            println!("------------- ------------- -------------     ------------- ------------- -------------");
            println!("------------- ------------- Graphics closed. Shutting down. ------------- -------------");
            println!("------------- ------------- -------------     ------------- ------------- -------------");
            std::process::exit(0);
        });
    }

    fn pull_newest_usable_state(&mut self) -> NetGameState {
        // Discards all states in the pipeline until empty, then uses the last one.
        let mut render_state = self.render_head_rec.recv().unwrap();

        let mut next_state_maybe = self.render_head_rec.try_recv();
        while next_state_maybe.is_ok(){
            render_state = next_state_maybe.unwrap();
            next_state_maybe = self.render_head_rec.try_recv();
        }
        render_state
    }
}

impl EventHandler for GraphicalIn {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let mut render_state = self.pull_newest_usable_state();
        render_state.render(ctx, self.my_player_id, &self.resources);

        let fps = timer::fps(ctx);
        let fps_display = Text::new(format!("FPS: {}", fps));
        // When drawing through these calls, `DrawParam` will work as they are documented.
        graphics::draw(
            ctx,
            &fps_display,
            (Point2::new(200.0, 0.0), graphics::WHITE),
        )?;



        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, repeat: bool) {
        if !repeat{
            let send_result = self.input_sink.send(InputChange::KeyDownUp(keycode, true));
            assert!(send_result.is_ok(), "Failed to take input: {:?}", send_result);
        }
    }
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.input_sink.send(InputChange::NewMousePosition(x, y)).unwrap();
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.input_sink.send(InputChange::NewMousePosition(x, y)).unwrap();
        self.input_sink.send(InputChange::MouseUpDown(button, false)).unwrap();
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.input_sink.send(InputChange::NewMousePosition(x, y)).unwrap();
        self.input_sink.send(InputChange::MouseUpDown(button, true)).unwrap();
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.input_sink.send(InputChange::KeyDownUp(keycode, false)).unwrap();
    }
}
