use ggez::*;
use ggez::{ContextBuilder, event};
use futures::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};

use crate::network::networking_structs::*;
use crate::players::inputs::*;
use ggez::event::{EventHandler, KeyMods};
use ggez::input::keyboard::KeyCode;

struct ClientMainState {
    game_state_head: GameState,
    game_state_tail: GameState,
    // Server socket.
    all_frames: Vec<InputsFrame>,
    my_player_id: PlayerID,
    messages_to_process: Arc<Mutex<Vec<i32>>>,
}
impl ClientMainState{
    pub fn new() -> ClientMainState{
        ClientMainState{
            game_state_head: GameState::new(),
            game_state_tail: GameState::new(),
            all_frames: vec![],
            my_player_id: -1,
            messages_to_process: Arc::new(Mutex::new(vec![]))
        }
    }
}




pub fn client_main() -> GameResult{
    println!("Starting as client.");


    let cb = ContextBuilder::new("Oh my literal pogger", "Atomsadiah")
        .window_setup(conf::WindowSetup::default().title("LiteralPoggyness"))
        .window_mode(conf::WindowMode::default().dimensions(500.0, 300.0)).add_resource_path("");

    let (ctx, events_loop) = &mut cb.build()?;

    let game = &mut ClientMainState::new();//ctx)?;

    let local_players_mutex_cloned = game.local_players.clone();
    let online_players_mutex_cloned = game.messages_to_process.clone();

    let (stdin_tx, stdin_rx) = mpsc::channel(0);
    let thread_ip_sending = target_ip.clone();
    thread::spawn(|| net_input_transfers::keep_sending_inputs(thread_ip_sending, stdin_tx, local_players_mutex_cloned));
    
    event::run(ctx, events_loop, game)
}

impl EventHandler for ClientMainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);


            { // Need to be explicit in where the mutex locks are dropped.
                let mut messages_this_frame = self.messages_to_process.lock().unwrap();
                for net_message in &*messages_this_frame{
                    match net_message{
                        NetMessageType::ConnectionInit(msg_init) => {
                            println!("Welcomed with a message: {}", msg_init.welcome_msg);
                            self.online_players.push(OnlinePlayer{
                                controller: PlayerController { input_state: InputState::new()}
                            });


                            let mut pending = PendingEntities::new();


                            let mut pending_entity_online_player = PendingEntity::new();
                            pending_entity_online_player.add_component(PositionComp{ x: 0.0, y: 0.0 });
                            pending_entity_online_player.add_component(VelocityComp{ x: 2.0, y: 1.0 });
                            pending_entity_online_player.add_component(SizeComp{ x: 50.0, y: 50.0 });
                            pending_entity_online_player.add_component(velocityWithInputComp{ owner_id: 2 });
                            pending_entity_online_player.add_component(RenderComp{ hue: graphics::Color::from_rgb(255,150,150) });
                            pending.create_entity(pending_entity_online_player);



                            self.world.update_entities(&mut self.storage, pending);



                        },
                        NetMessageType::InputsUpdate(msg_inputs) => {
                            // TODO - need to do some player ID matching here.
                            for online_player in &mut self.online_players{
                                online_player.controller = msg_inputs.controllers[0].clone();
                            }

                        },
                    };
                }
                messages_this_frame.clear();
            }


            let controllers = self.get_controllers_clone();

            let mut pending = PendingEntities::new();

            secret_position_system(&self.world, &mut pending, &mut self.storage.position_s, &mut self.storage.velocity_s);
            secret_velocity_system(&self.world, &mut pending, &mut self.storage.position_s, &mut self.storage.velocity_s);
            secret_velocityWithInput_system(&self.world, &mut pending, &mut self.storage.velocity_s,
                                            &mut self.storage.velocityWithInput_s, &controllers);

            self.world.update_entities(&mut self.storage, pending);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let mut pending = PendingEntities::new();

        secret_render_system(&self.world, &mut pending,
                             &mut self.storage.position_s,
                             &mut self.storage.render_s,
                             &mut self.storage.size_s,
                             ctx);



        self.world.update_entities(&mut self.storage, pending);

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
        self.keys_pressed.insert(keycode, true);

        match keycode {
            KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.keys_pressed.insert(keycode, false);
    }
}

