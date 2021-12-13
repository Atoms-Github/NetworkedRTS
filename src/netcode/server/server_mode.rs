use std::panic;
use std::sync::{Arc, RwLock};
use crossbeam_channel::*;
use std::thread;
use std::time::{SystemTime, Duration};
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use crate::netcode::*;
use crate::netcode::common::external_msg::*;
use crate::netcode::common::timekeeping::*;

use crate::netcode::*;
use crate::rts::GameState;
use bibble_tokio::{NetHubTop, OutMsg};
use crate::netcode::client::client_hasher::FramedHash;
use std::collections::HashSet;
use crate::netcode::common::confirmed_data::{ConfirmedData, SimDataPackage, SimDataOwner};
use crate::netcode::common::net_game_state::NetGameState;
use crate::netcode::client::header_threads::HEAD_AHEAD_FRAME_COUNT;


pub struct Server {
    net: NetHubTop<ExternalMsg>,
    data: ConfirmedData,
    game_state: NetGameState,
    known_frame_zero: KnownFrameInfo,
    init_box: HashSet<PlayerID>,
}


impl Server {
    pub fn start(hosting_ip: String){
        let net = bibble_tokio::start_server(hosting_ip.clone());
        let game_state = NetGameState::new();

        let mut server = Server {
            net,
            data: ConfirmedData::new(),
            game_state,
            known_frame_zero: KnownFrameInfo::new_from_args(0, SystemTime::now()),
            init_box: Default::default()
        };
        server.core_loop();
    }
    fn on_new_net_msg(&mut self, message: OutMsg<ExternalMsg>){
        match message{
            OutMsg::PlayerConnected(player_id) => {}
            OutMsg::PlayerDisconnected(player_id) => {
                log::info!("Player disconnected!");
                if self.init_box.remove(&player_id){
                    for package in self.data.server_disconnect_player(
                        player_id, self.game_state.get_simmed_frame_index()){
                        self.add_confirmed_data(package);
                    }
                }
            }
            OutMsg::NewFax(player_id, msg) => {
                match msg{
                    ExternalMsg::ConnectionInitQuery => {
                        log::info!("Received initialization request for player with ID: {}", player_id);

                        let game_state = self.game_state.clone();

                        let msg = NetMsgGreetingResponse {
                            assigned_player_id: player_id,
                            known_frame: self.known_frame_zero.clone(),
                            game_state,
                        };
                        let response = ExternalMsg::ConnectionInitResponse(msg);
                        self.net.send_msg(player_id, response, true);
                    },
                    ExternalMsg::WorldDownloaded{player_name, color} => {
                        if self.init_box.insert(player_id){
                            for package in self.data.server_connect_player(player_id, player_name, color){
                                self.add_confirmed_data(package);
                            }
                        }
                    },
                    ExternalMsg::GameUpdate(update_info) => {
                        if let SimDataPackage::PlayerInputs(data, player) = update_info{
                            let last_confirmed_input_maybe = self.data.get_last_input_frame(player);
                            let mut last_allowed_input = self.game_state.get_simmed_frame_index() + 1;
                            // Delete everything that's already been simulated.
                            // We'll still have overlap with existing data, but that's good, since we can check it.
                            let trimmed = data.trim_earlier(last_allowed_input);
                            self.add_confirmed_data(SimDataPackage::PlayerInputs(trimmed, player_id));
                        }else{
                            panic!("Server recced from server?");
                        }
                    },
                    ExternalMsg::InputQuery(query) => {
                        let owned_data = self.data.fulfill_query(&query, 20);
                        if owned_data.get_size() == 0{
                            log::info!("Failed to fulfil query {:?}", query);
                        }else{
                            log::debug!("Responded to {}'s req for {:?} with {:?} items", player_id, query, owned_data);
                            self.net.send_msg(player_id, ExternalMsg::GameUpdate(owned_data),false);
                        }
                    },
                    _ => {
                        panic!("Unexpected message");
                    }
                }
            }
        }
    }
    fn on_new_head_frame(&mut self, frame: FrameIndex){
        // Easy peasy when it comes to server's server events.
        // On head frame, send new one on head frame.
        // On event that needs scheduled, put it one frame after whatever we've got.
        {// Send new server event.
            if self.data.get_server_events(frame).is_none(){
                let new_data = SimDataPackage::new_single_server(frame, vec![]);
                self.add_confirmed_data(new_data);
            }
        }


        { // Fabricate missing data until we make as much progress as we need: (:
            loop{
                let data_query = self.game_state.sim_tail_far_as_pos(&self.data);
                // Actually, why are we bothering asking? The train goes on.
                // Lets just make it up.
                // The 'tail' frame doesn't exist. All it is is literally the cutoff point.
                let frames_to_get_input_in = HEAD_AHEAD_FRAME_COUNT;
                // If someone is too slow:
                if data_query.frame_offset < self.known_frame_zero.get_intended_current_frame() - frames_to_get_input_in{
                    if let SimDataOwner::Player(player) = data_query.query_type{
                        // Just make it up. :).
                        let their_last_inputs = self.data.get_last_input(player).cloned().unwrap_or_default();

                        let new_data_package = SimDataPackage::new_single_player(
                                data_query.frame_offset, player, their_last_inputs.clone());
                        log::info!("Fabricated missing inputs for {} on {}", player, data_query.frame_offset);

                        self.add_confirmed_data(new_data_package);
                    }else{
                        assert!(false, "Server shouldn't be waiting for the server ...")
                    }
                }else{
                    break;
                }
            }
        }
        { // Send hash:
            let hash = self.game_state.get_hash();
            self.net.send_msg_all(ExternalMsg::NewHash(hash), false);
        }
    }
    fn add_confirmed_data(&mut self, new_data: SimDataPackage){
        self.data.write_data(new_data.clone());
        self.net.send_msg_all(ExternalMsg::GameUpdate(new_data), false);
    }
    pub fn core_loop(mut self){
        let head_frames = self.known_frame_zero.start_frame_stream_from_now();
        loop{
            crossbeam_channel::select! {
                recv(self.net.up) -> msg =>{
                    self.on_new_net_msg(msg.unwrap());
                },
                recv(head_frames) -> new_frame =>{
                    self.on_new_head_frame(new_frame.unwrap());
                },
            };
        }
    }
}

pub fn server_main(hosting_ip: String){
    log::info!("Starting as server. Going to host on {}", hosting_ip);

    let server = Server::start(hosting_ip);

    log::info!("Server finished.");
}
