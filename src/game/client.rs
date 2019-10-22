use ggez::*;
use ggez::{ContextBuilder, event};
use futures::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::network::networking_structs::*;
use crate::network::networking_utils::*;
use crate::network::networking_message_types::*;
use crate::players::inputs::*;
use ggez::event::{EventHandler, KeyMods};
use ggez::input::keyboard::KeyCode;
use crate::game::client_networking::perform_handshake;
use tokio::net::TcpStream;
use tokio::io::WriteHalf;
use std::borrow::Borrow;

struct ClientMainState {
    game_state_head: GameState,
    game_state_tail: GameState,
    socket_write: WriteHalf<TcpStream>,
    all_frames: InputFramesStorage,
    my_player_id: PlayerID,
    client_message_box: MessageBox,
    network_oh_my_omies_mode: bool
}
impl ClientMainState{
    pub fn new(socket_write: WriteHalf<TcpStream>, my_player_id: PlayerID) -> ClientMainState{
        ClientMainState{
            game_state_head: GameState::new(),
            game_state_tail: GameState::new(),
            socket_write,
            all_frames: vec![],
            my_player_id,
            client_message_box: MessageBox::new(),
            network_oh_my_omies_mode: false
        }
    }
}

pub fn client_main(connection_target_ip: &String) -> GameResult{
    println!("Starting as client.");

    let cb = ContextBuilder::new("Oh my literal pogger", "Atomsadiah")
        .window_setup(conf::WindowSetup::default().title("LiteralPoggyness"))
        .window_mode(conf::WindowMode::default().dimensions(500.0, 300.0)).add_resource_path("");

    let (ctx, events_loop) = &mut cb.build()?;

    let mut handshake_result = perform_handshake(connection_target_ip);
    println!("Handshake successful.");

    let mut client_main_state = &mut ClientMainState::new(handshake_result.socket_write, handshake_result.player_id);//ctx)?;

    client_main_state.client_message_box.init_message_box_filling(&mut handshake_result.socket_read);
//    thread::spawn(|| net_input_transfers::keep_sending_inputs(thread_ip_sending, stdin_tx, local_players_mutex_cloned));
    
    event::run(ctx, events_loop, client_main_state)
}

impl EventHandler for ClientMainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);


            let messages_guard = Mutex::lock(&self.client_message_box.items).unwrap();


            for message in &*messages_guard{
                match message{
                    NetMessageType::ConnectionInitQuery(_) => {},
                    NetMessageType::InputsUpdate(inputs_update) => {
                        // TODO - fancy vector filter to remove all these after processing them.

                        let last_history_frame_index = self.all_frames.frames.len();
                        self.all_frames.blanks_up_to_index(last_frame_index);
                        for frame_index_to_twenty in 0..inputs_update.input_states.len() {
                            // TODO - Use fancy vector clone section method.

                            let frame_history_index = inputs_update.frame_index + frame_index_to_twenty;

                            self.all_frames.frames.get_mut(frame_history_index)
                                .unwrap().inputs.insert(inputs_update.player_id, inputs_update.input_states.get(frame_index_to_twenty).unwrap());
                        }
                    },
                    NetMessageType::ConnectionInitResponse(_) => {},
                }
            }


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


//let controllers = self.get_controllers_clone();
//
//let mut pending = PendingEntities::new();
//
//secret_position_system(&self.world, &mut pending, &mut self.storage.position_s, &mut self.storage.velocity_s);
//secret_velocity_system(&self.world, &mut pending, &mut self.storage.position_s, &mut self.storage.velocity_s);
//secret_velocityWithInput_system(&self.world, &mut pending, &mut self.storage.velocity_s,
//&mut self.storage.velocityWithInput_s, &controllers);
//
//self.world.update_entities(&mut self.storage, pending);



//
////            std::mem::drop();
//{ // Need to be explicit in where the mutex locks are dropped.
//let mut messages_this_frame = self.messages_to_process.lock().unwrap();
//for net_message in &*messages_this_frame{
//match net_message{
//NetMessageType::ConnectionInit(msg_init) => {
//println!("Welcomed with a message: {}", msg_init.welcome_msg);
//self.online_players.push(OnlinePlayer{
//controller: PlayerController { input_state: InputState::new()}
//});
//
//
//let mut pending = PendingEntities::new();
//
//
//let mut pending_entity_online_player = PendingEntity::new();
//pending_entity_online_player.add_component(PositionComp{ x: 0.0, y: 0.0 });
//pending_entity_online_player.add_component(VelocityComp{ x: 2.0, y: 1.0 });
//pending_entity_online_player.add_component(SizeComp{ x: 50.0, y: 50.0 });
//pending_entity_online_player.add_component(velocityWithInputComp{ owner_id: 2 });
//pending_entity_online_player.add_component(RenderComp{ hue: graphics::Color::from_rgb(255,150,150) });
//pending.create_entity(pending_entity_online_player);
//
//
//
//self.world.update_entities(&mut self.storage, pending);
//
//
//
//},
//NetMessageType::InputsUpdate(msg_inputs) => {
//// TODO - need to do some player ID matching here.
//for online_player in &mut self.online_players{
//online_player.controller = msg_inputs.controllers[0].clone();
//}
//
//},
//};
//}
//messages_this_frame.clear();
//}