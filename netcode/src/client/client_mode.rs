use crossbeam_channel::*;
use std::thread;
use std::time::{Duration};

use crate::client::connect_net_seg::*;
use crate::client::graphical_seg::*;
use crate::client::header_threads::*;
use crate::common::external_msg::*;
use crate::common::timekeeping::*;
use ggez::input::keyboard::KeyCode;
use crate::pub_types::{FrameIndex, PlayerID, Shade, HashType};
use ggez::input::gamepad::gamepad;
use std::sync::Arc;
use crate::client::client_data_store::ClientDataStore;
use std::collections::HashMap;
use crate::client::client_hasher::ClientHasher;
use crate::common::net_game_state::{NetGameState, GameState};
use crate::common::input_state::{InputState, InputChange};
use crate::common::superstore_seg::SuperstoreData;
use crate::common::confirmed_data::{SimDataQuery, SimDataOwner};


pub struct Client<T : 'static + GameState> {
    player_id: PlayerID,
    net: ClientNet<T>,
    player_name: String,
    color: Shade,
    data: ClientDataStore,
    hasher: ClientHasher,
    game_state: NetGameState<T>,
    head_handle: Sender<HeadSimPacket<T>>,
    curret_input: InputState,
    known_frame: KnownFrameInfo,
}
impl<T : 'static + GameState> Client<T> {
    pub fn go(player_name: String, color: Shade, connection_ip: String, preferred_port: i32){
        log::info!("Starting as client.");

        let mut net = ClientNet::<T>::start(connection_ip.clone());
        let mut welcome_info = net.get_synced_greeting();
        let my_id = welcome_info.assigned_player_id;
        log::info!("Downloaded game state which has simmed {}", welcome_info.game_state.get_simmed_frame_index());

        net.client.send_msg(ExternalMsg::WorldDownloaded { player_name: player_name.clone(), color }, true);

        let mut data = ClientDataStore::new(welcome_info.assigned_player_id);

        let (tx_head, rx_head) = unbounded();
        let (tx_input, rx_input) = unbounded();

        let known_frame = welcome_info.known_frame.clone();
        let mut client = Client{
            player_id: welcome_info.assigned_player_id,
            net,
            player_name,
            color,
            data,
            hasher: ClientHasher::new(),
            game_state: welcome_info.game_state,
            head_handle: tx_head,
            curret_input: Default::default(),
            known_frame
        };
        thread::spawn(move ||{
            client.core_loop(rx_input);
        });
        HeaderThread::start(tx_input, rx_head, my_id);
    }
    fn on_new_head_frame(&mut self, head_frame: FrameIndex){
        let old_tail_frame = self.game_state.get_simmed_frame_index();

        let issue = self.game_state.sim_tail_far_as_pos(&self.data.confirmed_data);
        let new_tail_frame = self.game_state.get_simmed_frame_index();
        let did_trail_progress = new_tail_frame != old_tail_frame;

        // I.e. we'll request data if we can't even make it to here.
        // The lower this is, the more work we're happy to do before complaining.
        let intended_tail = head_frame - HEAD_AHEAD_FRAME_COUNT;
        if new_tail_frame < intended_tail{
            self.net.client.send_msg(ExternalMsg::InputQuery(issue), false);
        }
        if did_trail_progress{
            // Send head state:
            let gamestate = self.game_state.clone();
            let sim_data = self.data.get_head_sim_data(
                gamestate.get_simmed_frame_index() + 1, head_frame /* Try and go all the way */);
            let head_packet = HeadSimPacket{
                game_state: gamestate,
                sim_data
            };
            self.head_handle.send(head_packet).unwrap();
        }
        self.hasher.add_framed(self.game_state.get_hash());

        self.data.predicted_local.write_data(SuperstoreData{
            data: vec![self.curret_input.clone()],
            frame_offset: head_frame - 1
        });
        let my_last_20 = self.data.fulfill_query(&SimDataQuery{
            query_type: SimDataOwner::Player(self.player_id),
            frame_offset: head_frame - HEAD_AHEAD_FRAME_COUNT
        }, 20);
        self.net.client.send_msg(ExternalMsg::GameUpdate(my_last_20), false);
    }
    fn on_new_net_msg(&mut self, message: ExternalMsg<T>){
        match message{
            ExternalMsg::GameUpdate(update) => {
                self.data.confirmed_data.write_data(update);
            },
            ExternalMsg::InputQuery(query) => {
                let owned_data = self.data.fulfill_query(&query, 20);
                if owned_data.get_size() == 0{
                    log::info!("Failed to fulfil query {:?}", query);
                }else{
                    log::info!("Responded to server req for {:?} with {:?} items", query, owned_data);
                    self.net.client.send_msg(ExternalMsg::GameUpdate(owned_data), false);

                }
            },
            ExternalMsg::PingTestResponse(_) => {
                // Do nothing. Doesn't matter that intro stuff is still floating when we move on.
            }
            ExternalMsg::NewHash(framed_hash) => {
                self.hasher.add_framed(framed_hash);
            },
            _ => {
                panic!("Client shouldn't be getting a message of this type (or at this time)!")
            }
        }
    }
    fn on_new_input(&mut self, input: InputChange){
        input.apply_to_state(&mut self.curret_input);
    }

    pub fn core_loop(mut self, inputs: Receiver<InputChange>){
        let head_frames = self.known_frame.start_frame_stream_from_now();
        loop{
            crossbeam_channel::select! {
                recv(self.net.client.up) -> msg =>{
                    self.on_new_net_msg(msg.unwrap());
                },
                recv(head_frames) -> new_frame =>{
                    self.on_new_head_frame(new_frame.unwrap());
                },
                recv(inputs) -> new_input =>{
                    self.on_new_input(new_input.unwrap());
                }
            };
        }
    }

}








