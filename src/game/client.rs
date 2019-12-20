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


struct ClientMainState {
    game_state_head: GameState,
    game_state_tail: GameState,
    socket: TcpStream,
    all_frames: InputFramesStorage,
    my_player_id: PlayerID,
    client_message_box: MessageBox,
    network_oh_my_homies_mode: bool,
    my_current_input_state: InputState,
    known_frame_info: KnownFrameInfo,
    debug_client_zero: SystemTime,
    last_simed_head_frame: usize
}
impl ClientMainState{
    pub fn new(socket: TcpStream, message_box: MessageBox, state_tail: GameState, my_player_id: PlayerID, known_frame_info: KnownFrameInfo) -> ClientMainState{

        ClientMainState{
            game_state_head: GameState::new(),
            game_state_tail: state_tail,
//            socket_write: FramedWrite::new(socket_write, dans_codec::Bytes),
            socket,
            all_frames: InputFramesStorage::new(),
            my_player_id,
            client_message_box: message_box,
            network_oh_my_homies_mode: false,
            my_current_input_state: InputState::new(),
            known_frame_info,
            debug_client_zero: SystemTime::now(),
            last_simed_head_frame: 0 // Dan's game.
        }
    }
}



pub fn client_main(connection_target_ip: &String){
    let local_connection_target_ip = connection_target_ip.clone();
    println!("Starting as client.");

    let server_connection_init = connect_and_send_handshake(&local_connection_target_ip);
    client_main_loop(server_connection_init);


}

fn client_main_loop(init_response: ConnectToServerInit){
    let cb = ContextBuilder::new("Oh my literal pogger", "Atomsadiah")
        .window_setup(conf::WindowSetup::default().title("LiteralPoggyness"))
        .window_mode(conf::WindowMode::default().dimensions(500.0, 300.0)).add_resource_path(""); // TODO: Find what resource path.

    let (ctx, events_loop) = &mut cb.build().unwrap();


    loop{

        let mut msgs = init_response.msg_box.items;

    }

    let welcome_messages = init_response.welcome_messages_channel;
    let welcome_message = welcome_messages.recv().unwrap();

    let my_player_id;
    let server_tail;
    let gathered_frames;
    let known_frame_info;

    match welcome_message{
        NetMessageType::ConnectionInitResponse(response) => {
            println!("Read handshake. My player ID is: {}", response.assigned_player_id);
            my_player_id = response.assigned_player_id;
            server_tail = response.game_state;
            gathered_frames = response.frames_gathered_so_far;
            known_frame_info = response.known_frame_info;
        },
        _ => {
            panic!("Why/how was a non connection init message sent in the welcome messages channel?");
        },
    }
    println!("Meme2 {}", my_player_id);

    let message_box = MessageBox::new();
    // Normal messages can fill up all it wants while we're waiting for the welcome message.
    // Once the welcome is recieved, normal ones can start to be processed.
    message_box.spawn_thread_fill_from_receiver(init_response.normal_messages_channel);
    message_box.spawn_thread_read_cmd_input();

    let client_main_state = &mut ClientMainState::new(init_response.stream, message_box, server_tail, my_player_id, known_frame_info);//ctx)?;
    println!("Gathered frames length: {} start_index: {}", gathered_frames.frames_section.len(), gathered_frames.start_index);
    client_main_state.all_frames.insert_frames_partial(gathered_frames);
//    client_main_state.client_message_box.(handshake_result_future.socket_read);

    let result = event::run(ctx, events_loop, client_main_state);
}

impl EventHandler for ClientMainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
//        println!("BeforeUpdate {:?}", SystemTime::now().duration_since(self.debug_client_zero).unwrap());
        const DESIRED_FPS: u32 = 60;
//        while timer::check_update_time(ctx, DESIRED_FPS) {
        let seconds = 1.0 / (DESIRED_FPS as f32);

        let target_frame_tail = self.known_frame_info.get_intended_current_frame();
        let target_frame_head = target_frame_tail + 19;

        self.all_frames.blanks_up_to_index(target_frame_head); // TODO: Should detect and handle when inputs don't come in.
        // Fill new blank created with my current inputs.

        for frame_index in self.last_simed_head_frame..(target_frame_head){ // In the case of lag (e.g. graphics starting lag), set all missing frame's inputs to current input.
            self.all_frames.frames.get_mut(frame_index).unwrap().inputs.insert(self.my_player_id, self.my_current_input_state.clone());
//            println!("Added my frame info for frame number:  {}", frame_index);
        }
        // Now that we've calculated local inputs, now we send them to server.

        let mut inputs : [InputState; 20] = [InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/InputState::new(), /*OhMyPoggy*/]; // TODO: Make faster (This is unnecessary).
        for index in target_frame_tail..target_frame_head{
//            println!("Tail: {} Head: {} Size: {}", target_frame_tail, target_frame_head, self.all_frames.frames.len());
            inputs[index - target_frame_tail /*Dans*/] = self.all_frames.frames.get(index).unwrap().inputs.get(&self.my_player_id).unwrap().clone(); // Game of Dan.
        }
        let inputs_update = NetMessageType::InputsUpdate(NetMsgInputsUpdate{
            player_id: self.my_player_id,
            frame_index: target_frame_tail,
            input_states: inputs
        });
        let serialized = bincode::serialize(&inputs_update).unwrap();
        println!("Sending size: {}", serialized.len());
//        self.socket_write.writev(serialized);
        self.socket_write.write(&serialized[..]).unwrap();
        self.socket_write.flush().unwrap();
        self.socket_write.write(&serialized[..]).unwrap();
        self.socket_write.flush().unwrap();



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
                },
                NetMessageType::NewPlayer(new_player_info) => {
                    self.game_state_tail.add_player(new_player_info.player_id);
                    self.all_frames.add_player_default_inputs(&new_player_info.player_id, new_player_info.frame_added)
                }
            }
        }
        while self.game_state_tail.last_frame_simed < target_frame_tail{
            let frame_index_to_simulate = self.game_state_tail.last_frame_simed;

            let inputs_to_use = self.all_frames.frames.get(frame_index_to_simulate).expect("Panic! Required frames haven't arrived yet. OH MY HOMIES!");
            self.game_state_tail.simulate_tick(inputs_to_use, 0.016 /* TODO: Use real delta. */);
            self.game_state_tail.last_frame_simed += 1;
        }
        self.game_state_head = self.game_state_tail.clone();

        while self.game_state_head.last_frame_simed < target_frame_head{
            let frame_index_to_simulate = self.game_state_head.last_frame_simed;
//                println!("Simulating frame nubmer {}", frame_index_to_simulate);

            let possible_arrived_inputs = self.all_frames.frames.get(frame_index_to_simulate);
            let inputs_to_use;
            let blank_inputs = InputsFrame::new();
            match possible_arrived_inputs{
                Some(inputs) => {
                    inputs_to_use = inputs;
                }
                None=> {
                    inputs_to_use = &blank_inputs; // TODO: Should 1. use the last known input, not nothing. And 2. should split inputs by players, so only unknown players are guessed.
                }
            }
            self.game_state_head.simulate_tick(inputs_to_use, 0.016 /* TODO: Use real delta. */);

            self.game_state_head.last_frame_simed += 1;
        }
        self.last_simed_head_frame = target_frame_head;
//        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        // TODO things are popping assuming that the draw pretty much always happens straight after update.
        // TODO we need to make a thing to allow logic to kinda lag while still updating render.
        graphics::clear(ctx, graphics::BLACK);


        let mut pending = PendingEntities::new();

        secret_render_system(&self.game_state_head.world, &mut pending,
                             &mut self.game_state_head.storages.position_s,
                             &mut self.game_state_head.storages.render_s,
                             &mut self.game_state_head.storages.size_s,
                             ctx);

        self.game_state_head.world.update_entities(&mut self.game_state_head.storages, pending); // TODO ask Richard if this is really needed after calling render. (Unlikely)

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

        self.my_current_input_state.set_keycode_pressed(keycode, true);

        match keycode {
            KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.my_current_input_state.set_keycode_pressed(keycode, false);
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