use crossbeam_channel::*;
use std::thread;

use ggez::*;
use ggez::{ContextBuilder, event};
use ggez::event::{EventHandler, KeyMods, MouseButton, EventLoop};
use ggez::input::keyboard::KeyCode;

use crate::pub_types::*;
use crate::*;
use std::time::SystemTime;
use ggez::graphics::Text;
use std::collections::BTreeMap;
use nalgebra::Point2;
use std::sync::Arc;
use std::path::Path;
use std::borrow::Borrow;
use ggez::conf::{WindowMode, FullscreenType};
use crate::common::net_game_state::{NetGameState, GameState};
use crate::common::input_state::InputChange;

pub struct GraphicalIn<T : 'static + GameState> {
    render_head_rec: Receiver<NetGameState<T>>,
    my_player_id: PlayerID,
    input_sink: Sender<InputChange>,
    texts: BTreeMap<&'static str, Text>,
    resources: Option<T::Resources>, // Yes, this can be refactored.
    fullscreen: bool,
    window_mode: conf::WindowMode,
}

impl<T : 'static + GameState> GraphicalIn<T> {
    pub fn start(inputs: Sender<InputChange>, head_render_handle: Receiver<NetGameState<T>>, my_player_id: PlayerID) -> !{
        let mut texts = BTreeMap::new();
        let text = Text::new("Hello, World!");
        // Store the text in `App`s map, for drawing in main loop.
        texts.insert("0_hello", text);


        let mut window_mode = WindowMode::default();

        window_mode = window_mode.dimensions(1440.0, 810.0);
        window_mode.resizable = true;
        // window_mode.resize_on_scale_factor_change = true;
        let mut graphical_in = GraphicalIn{
            render_head_rec: head_render_handle,
            my_player_id,
            input_sink: inputs,
            texts,
            resources: None,
            fullscreen: false,
            window_mode
        };


        let cb = ContextBuilder::new("Oh my literal pogger", "Atoms")
            .window_setup(conf::WindowSetup::default().title("LiteralPoggyness")).window_mode(graphical_in.window_mode.clone())
            .add_resource_path("");

        let (mut ctx, events_loop) = cb.build().unwrap();


        graphical_in.resources = Some(T::gen_resources(&mut ctx));
        event::run(ctx, events_loop, graphical_in);
    }
}

impl<T : 'static + GameState> EventHandler<GameError> for GraphicalIn<T> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(50,50,50));
        let mut render_state = crate::utils::pull_latest(&mut self.render_head_rec);
        // SIMULATE 20.5 FRAMES OF HEAD TO CALCULATE HEAD
        render_state.render(ctx, self.my_player_id, &mut self.resources.as_mut().unwrap());

        let fps = timer::fps(ctx);
        let fps_display = Text::new(format!("FPS: {}", fps));
        // When drawing through these calls, `DrawParam` will work as they are documented.
        graphics::draw(
            ctx,
            &fps_display,
            (Point2::new(200.0, 0.0), graphics::Color::WHITE),
        )?;




        graphics::present(ctx)?;

        Ok(())
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        self.input_sink.send(InputChange::MouseWheelEvent(y)).unwrap();
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
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.input_sink.send(InputChange::NewMousePosition(x, y)).unwrap();
    }
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(
            0.0,
            0.0,
            width as f32,
            height as f32,
        );
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, repeat: bool) {
        if !repeat{
            if keycode == KeyCode::F11{
                self.fullscreen = !self.fullscreen;
                let window = graphics::window(ctx);
                if self.fullscreen{
                    self.window_mode.fullscreen_type = FullscreenType::Desktop;
                }else{
                    self.window_mode.fullscreen_type = FullscreenType::Windowed;
                }

                self.window_mode.maximized = self.fullscreen;
                self.window_mode.borderless = self.fullscreen;

                graphics::set_mode(ctx, self.window_mode.clone()).unwrap();
            }


            let send_result = self.input_sink.send(InputChange::KeyDownUp(keycode, true));
            assert!(send_result.is_ok(), "Failed to take input: {:?}", send_result);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.input_sink.send(InputChange::KeyDownUp(keycode, false)).unwrap();
    }
}
