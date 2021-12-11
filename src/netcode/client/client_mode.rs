use crossbeam_channel::*;
use std::thread;
use std::time::{Duration};

use crate::netcode::client::connect_net_seg::*;
use crate::netcode::client::graphical_seg::*;
use crate::netcode::client::logic_sim_header_seg::*;
use crate::netcode::common::logic::logic_sim_tailer_seg::*;
use crate::netcode::common::network::external_msg::*;
use crate::netcode::common::sim_data::input_state::*;
use crate::netcode::common::sim_data::confirmed_data::*;
use crate::netcode::common::time::scheduler_segment::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::common::sim_data::superstore_seg::*;
use crate::netcode::client::input_handler_seg::*;
use ggez::input::keyboard::KeyCode;
use crate::netcode::server::net_hub_front_seg::NetHubFrontMsgIn;
use crate::pub_types::{FrameIndex, PlayerID, Shade, HashType};
use ggez::input::gamepad::gamepad;
use crate::rts::GameState;
use std::sync::Arc;
use crate::netcode::client::client_data_store::ClientDataStore;
use std::collections::HashMap;

enum ClientMsg{
    NewFrame,
    NewNetMsg(ExternalMsg),
    NewInput,
}

struct Client {
    welcome_info: NetMsgGreetingResponse,
    seg_connect_net: ClientNet,
    player_name: String,
    color: Shade,
    data: ClientDataStore,
    hashes: HashMap<FrameIndex, HashType>,
    seg_logic_tailer: LogicSimTailer,
    head_handle: Sender<HeadSimPacket>
}
impl Client {
    pub fn go(player_name: String, color: Shade, connection_ip: String, preferred_port: i32){
        log::info!("Starting as client.");

        let mut seg_connect_net = ClientNet::start(connection_ip.clone());
        let mut welcome_info = seg_connect_net.get_synced_greeting();
        log::info!("Downloaded game state which has simmed {}", welcome_info.game_state.get_simmed_frame_index());

        let downloaded_msg = ExternalMsg::WorldDownloaded(WorldDownloadedInfo{
            player_name: connected_client.player_name.clone(),
            color: connected_client.color.clone()
        });
        connected_client.seg_connect_net.net_sink.send((downloaded_msg, true)).unwrap();
        seg_connect_net.client.send_msg(ExternalMsg::ConnectionInitQuery);

        let mut seg_data_storage = ConfirmedData::new(first_frame_to_store);

        // Init storage for all existing players. (they won't get inited by a ServerEvent.)
        for connected_player in welcome_info.game_state.get_connected_players(){
            seg_data_storage.add_new_player(connected_player, self.welcome_info.game_state.get_simmed_frame_index() + 1);
        }

        let known_frame = welcome_info.known_frame.clone();
        let mut client = Client{
            welcome_info,
            seg_connect_net,
            player_name,
            color,
            data: ClientDataStore::new(),
            hashes: Default::default(),
            seg_logic_tailer: LogicSimTailer {
                game_state: welcome_info.game_state,
            },
            head_handle: Sender<HeadSimPacket>,
        };
        thread::spawn(||{
            client.core_loop();
        });
        // TODO: Do head.gographics with main thread.
    }
    pub fn core_loop(mut self){
        while let Ok(item) = connected_client.seg_connect_net.net_rec.as_ref().unwrap().try_recv(){
            match item{
                ExternalMsg::GameUpdate(update) => {
                    if crate::DEBUG_MSGS_MAIN {
                        log::debug!("Net rec message: {:?}", update);
                    }
                    // log::info!("Client Leart: {:?}", update);
                    self.seg_data_storage.write_data(update);
                },
                ExternalMsg::InputQuery(query) => {
                    let owned_data = self.seg_data_storage.fulfill_query(&query, 20);
                    if owned_data.get_size() == 0{
                        log::info!("Failed to fulfil query {:?}", query);
                    }else{
                        log::info!("Responded to server req for {:?} with {:?} items", query, owned_data);
                        connected_client.seg_connect_net.net_sink.send((ExternalMsg::GameUpdate(owned_data), false)).unwrap();

                    }
                },
                ExternalMsg::PingTestResponse(_) => {
                    // Do nothing. Doesn't matter that intro stuff is still floating when we move on.
                }
                ExternalMsg::NewHash(framed_hash) => {
                    self.seg_logic_tailer.add_hash(framed_hash);
                },
                _ => {
                    panic!("Client shouldn't be getting a message of this type (or at this time)!")
                }
            }
        }
        // Also do:
        let known_frame = connected_client.welcome_info.known_frame.clone();

        let time_syncer = known_frame.start_frame_stream_from_now();
        let graphical_in = self.seg_graphical_in.take().unwrap();
        thread::spawn(move ||{
            loop{
                // Shouldn't need to use this.
                let _ = time_syncer.recv().unwrap();

                let mut tail_progress_made = false;

                self.update_net_rec(&mut connected_client);

                let tail_attempt_start = self.seg_logic_tailer.game_state.get_simmed_frame_index() + 1;
                let tail_attempt_end = (known_frame.get_intended_current_frame()).min(tail_attempt_start + 5);
                for tail_frame_attempt in tail_attempt_start..tail_attempt_end{ // TODO2: A number depending on processing time.
                    self.seg_input_handler.update(&mut self.seg_data_storage, self.seg_logic_tailer.game_state.get_simmed_frame_index() + HEAD_AHEAD_FRAME_COUNT);
                    // log::info!("Client to sim {}.", tail_frame_attempt);
                    match self.seg_logic_tailer.catchup_simulation(&self.seg_data_storage, tail_frame_attempt){
                        Some(missing_datas) => {
                            for missing_data in missing_datas{
                                // TODO1 - save up a bit, jees.
                                log::info!("Client missing data: {:?}", missing_data);
                                connected_client.seg_connect_net.net_sink.send((ExternalMsg::InputQuery(missing_data), false)).unwrap();
                            }
                            break; // No more chance of stuff.
                        }
                        None => {
                            // Tail sim successful.
                            tail_progress_made = true;

                        }
                    }
                }
                if tail_progress_made{
                    self.seg_logic_header.send_head_state(self.seg_logic_tailer.game_state.clone(), &self.seg_data_storage);
                }
            }
        });
    }

}








