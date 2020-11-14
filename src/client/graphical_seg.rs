use crossbeam_channel::*;
use std::thread;

use ggez::*;
use ggez::{ContextBuilder, event};
use ggez::event::{EventHandler, KeyMods, MouseButton};
use ggez::input::keyboard::KeyCode;

use crate::common::gameplay::ecs::world::*;
use crate::common::gameplay::game::game_state::*;
use crate::common::sim_data::input_state::*;
use crate::common::gameplay::systems::render::*;
use std::time::SystemTime;
use crate::common::types::*;

pub struct GraphicalSeg {
//    my_current_input_state: Arc<Mutex<InputState>>,
render_head_rec: Receiver<GameState>,
    my_player_id: PlayerID,
    sender: Option<Sender<InputChange>> // TODO3: Hmm. Option?
}
impl GraphicalSeg {
    pub fn new(head_render_handle: Receiver<GameState>, my_player_id: PlayerID) -> GraphicalSeg {
        GraphicalSeg {
            render_head_rec: head_render_handle,
            my_player_id,
            sender: None,
        }
    }

    pub fn start(mut self) -> Receiver<InputChange>{
        let (sender,receiver) = unbounded();

        self.sender = Some(sender);

        let cb = ContextBuilder::new("Oh my literal pogger", "Atomsadiah")
            .window_setup(conf::WindowSetup::default().title("LiteralPoggyness"))
            .window_mode(conf::WindowMode::default().dimensions(500.0, 300.0)).add_resource_path(""); // TODO3: Find what resource path.




        thread::spawn(move ||{
            let (ctx, events_loop) = &mut cb.build().unwrap();

            let mut meme = self;
            event::run(ctx, events_loop, &mut meme).unwrap();
        });


        receiver
    }
    fn render_state(&self, state: &mut GameState, ctx: &mut Context){
        secret_render_system(&state.world, &mut PendingEntities::new(),
                             &mut state.storages.position_s,
                             &mut state.storages.render_s,
                             &mut state.storages.size_s,
                             ctx);
    }
    fn pull_newest_usable_state(&mut self) -> GameState{
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

impl EventHandler for GraphicalSeg {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }





    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let mut render_state = self.pull_newest_usable_state();
        self.render_state(&mut render_state, ctx);



        graphics::present(ctx)?;

        Ok(())

    }



    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, repeat: bool) {
        if !repeat{
            let send_result = self.sender.as_ref().unwrap().send(InputChange::KeyDownUp(keycode, true));
            assert!(send_result.is_ok(), format!("Failed to take input: {:?}", send_result));
        }
    }
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.sender.as_ref().unwrap().send(InputChange::NewMousePosition(x, y)).unwrap();
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.sender.as_ref().unwrap().send(InputChange::NewMousePosition(x, y)).unwrap();
        self.sender.as_ref().unwrap().send(InputChange::MouseUpDown(button, false)).unwrap();
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.sender.as_ref().unwrap().send(InputChange::NewMousePosition(x, y)).unwrap();
        self.sender.as_ref().unwrap().send(InputChange::MouseUpDown(button, true)).unwrap();
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.sender.as_ref().unwrap().send(InputChange::KeyDownUp(keycode, false)).unwrap();
    }
}
