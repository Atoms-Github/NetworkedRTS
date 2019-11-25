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
use tokio::net::TcpStream;
use tokio::io::WriteHalf;

use crate::systems::render::*;
use futures::future::lazy;

use crate::ecs::world::*;
use crate::ecs::system_macro::*;

use crate::game::client_networking::*;

use futures::future::Future;
use tokio_threadpool::ThreadPool;
use futures::future::poll_fn;

struct ClientMainState {
    game_state_head: GameState,
    game_state_tail: GameState,
    socket_write: WriteHalf<TcpStream>,
    all_frames: InputFramesStorage,
    my_player_id: PlayerID,
    client_message_box: MessageBox,
    network_oh_my_omies_mode: bool,
    game_started: bool,
    my_current_input_state: InputState,
}
impl ClientMainState{
    pub fn new(socket_write: WriteHalf<TcpStream>, message_box: MessageBox, state_tail: GameState, my_player_id: PlayerID) -> ClientMainState{
        ClientMainState{
            game_state_head: GameState::new(),
            game_state_tail: state_tail,
            socket_write,
            all_frames: InputFramesStorage::new(),
            my_player_id,
            client_message_box: message_box,
            network_oh_my_omies_mode: false,
            my_current_input_state: InputState::new(),
            game_started: false
        }
    }
}



pub fn client_main(connection_target_ip: &String){
    let local_connection_target_ip = connection_target_ip.clone();
    
    tokio::run(futures::lazy(move || {
        println!("Starting as client.");



        let handshake_result_future = connect_and_send_handshake(&local_connection_target_ip);
        let spicy_task = handshake_result_future.map_err(|e|{
            println!("Error yote.");
        }).and_then(|handshake_response|{


            thread::spawn(||{
                client_main_loop(handshake_response);
            });
            Ok(())
        });

        tokio::spawn(spicy_task);
        Ok(())
    }));
}

fn client_main_loop(handshake_response: HandshakeResponse){
    let cb = ContextBuilder::new("Oh my literal pogger", "Atomsadiah")
        .window_setup(conf::WindowSetup::default().title("LiteralPoggyness"))
        .window_mode(conf::WindowMode::default().dimensions(500.0, 300.0)).add_resource_path("");

    let (ctx, events_loop) = &mut cb.build().unwrap();


    let welcome_messages = handshake_response.welcome_messages_channel;
    let welcome_message = welcome_messages.recv().unwrap();

    let my_player_id;
    let server_tail;
    let gathered_frames;

    match welcome_message{
        NetMessageType::ConnectionInitResponse(response) => {
            println!("Read handshake. My player ID is: {}", response.assigned_player_id);
            my_player_id = response.assigned_player_id;
            server_tail = response.game_state;
            gathered_frames = response.frames_gathered_so_far;
        },
        _ => {
            panic!("Why/how was a non connection init message sent in the welcome messages channel?");
        },
    }
    println!("Meme2 {}", my_player_id);



    let message_box = MessageBox::new();
    // Normal messages can fill up all it wants while we're waiting for the welcome message.
    // Once the welcome is recieved, normal ones can start to be processed.
    message_box.spawn_thread_fill_from_receiver(handshake_response.normal_messages_channel);
    message_box.spawn_thread_read_cmd_input();

    let client_main_state = &mut ClientMainState::new(handshake_response.socket_write,message_box,server_tail, my_player_id);//ctx)?;
    client_main_state.all_frames.insert_frames_partial(gathered_frames);
//    client_main_state.client_message_box.(handshake_result_future.socket_read);

    let result = event::run(ctx, events_loop, client_main_state);
}

impl EventHandler for ClientMainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
            let mut messages_guard = Mutex::lock(&self.client_message_box.items).unwrap();

            for message in (*messages_guard).drain(..){
                println!("MessageInMessageBox: {:?}", message);
                match message{
                    NetMessageType::ConnectionInitQuery(_) => {
                        panic!("Client shouldn't be asked for connection inits querys.");
                    },
                    NetMessageType::InputsUpdate(inputs_update) => {
                        self.all_frames.insert_frames(inputs_update.player_id,inputs_update.frame_index, &inputs_update.input_states);
                    },
                    NetMessageType::ConnectionInitResponse(_) => {
                        panic!("This should be in welcome channel.");
                    },
                    NetMessageType::LocalCommand(item) => {
                        println!("I've heard a command. Let me listen: {}", item.command);
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let mut pending = PendingEntities::new();

        secret_render_system(&self.game_state_head.world, &mut pending,
                             &mut self.game_state_head.storages.position_s,
                             &mut self.game_state_head.storages.render_s,
                             &mut self.game_state_head.storages.size_s,
                             ctx);

        self.game_state_head.world.update_entities(&mut self.game_state_head.storages, pending); // TODO ask Richard if this is really needed after calling render.

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

        self.my_current_input_state.keys_pressed.insert(keycode as usize, true);

        match keycode {
            KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.my_current_input_state.keys_pressed.insert(keycode as usize, false);
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