use crossbeam_channel::*;
use std::thread;
use std::time::{Duration};

use crate::netcode::client::connect_net_seg::*;
use crate::netcode::client::graphical_seg::*;
use crate::netcode::client::logic_sim_header_seg::*;
use crate::netcode::common::logic::logic_sim_tailer_seg::*;
use crate::netcode::common::network::external_msg::*;
use crate::netcode::common::sim_data::input_state::*;
use crate::netcode::common::sim_data::sim_data_storage::*;
use crate::netcode::common::time::scheduler_segment::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::common::sim_data::superstore_seg::*;
use crate::netcode::client::input_handler_seg::*;
use ggez::input::keyboard::KeyCode;
use crate::netcode::server::net_hub_front_seg::NetHubFrontMsgIn;
use crate::pub_types::{FrameIndex, PlayerID, Shade};
use ggez::input::gamepad::gamepad;
use crate::rts::GameState;
use std::sync::Arc;

pub struct ClientApp{
    player_name: String,
    color: Shade,
    connection_ip: String,
    preferred_id: i32,
}
impl ClientApp{
    pub fn go(player_name: String, color: Shade, connection_ip: String, preferred_id: i32){
        // Steps are:
        // 1. Init connection and download world.
        // 2. Do pre interesting.
        // 3. Create segs, and pass in init frame.
        // 4. Do post interesting.
        //
        // 1 is executed by the app and results in a connected client.
        // 2 and 3 are executed by the connected client and result in a ClientEx.
        // 4 is executed on the ClientEx.
        //
        log::info!("Starting as client.");
        let mut app = ClientApp{
            player_name,
            color,
            connection_ip,
            preferred_id
        };
        let connected_client = app.init_connection();
        connected_client.start();
    }
    fn init_connection(self) -> ConnectedClient{

        let mut seg_connect_net = ConnectNetEx::start(self.connection_ip.clone());

        let greeting = NetMsgGreetingQuery {};

        let mut welcome_info = seg_connect_net.get_synced_greeting(greeting);
        log::info!("Downloaded game state which has simmed {}", welcome_info.game_state.get_simmed_frame_index());

        return ConnectedClient{
            welcome_info,
            seg_connect_net,
            player_name: self.player_name.to_string(),
            color: self.color
        }
    }
}
struct ConnectedClient{
    welcome_info: NetMsgGreetingResponse,
    seg_connect_net: ConnectNetEx,
    player_name: String,
    color: Shade,
}
impl ConnectedClient{
    pub fn start(mut self){
        let my_init_frame = self.pre_interesting();
        let ex = self.init_segs(my_init_frame);
        ex.post_interesting(self, my_init_frame);
    }
    fn pre_interesting(&self) -> FrameIndex{
        let my_init_frame = self.welcome_info.known_frame.get_intended_current_frame() + 50; // modival How far in the future to plonk yourself.
        // log::debug!("I'm gonna init me on {}", my_init_frame);
        return my_init_frame;
    }
    fn init_segs(&mut self, my_init_frame: FrameIndex) -> ClientEx{

        let welcome_info = self.welcome_info.clone();

        let first_frame_to_store = welcome_info.game_state.get_simmed_frame_index() + 1;
        let mut seg_data_storage = SimDataStorage::new(first_frame_to_store);

        // Init storage for all existing players. (they won't get inited by a ServerEvent.)
        for connected_player in self.welcome_info.game_state.get_connected_players(){
            seg_data_storage.add_new_player(connected_player, self.welcome_info.game_state.get_simmed_frame_index() + 1);
        }
        let seg_scheduler = SchedulerSegEx::start(welcome_info.known_frame.clone());
        let mut seg_logic_tailer = LogicSimTailer::new(welcome_info.game_state, welcome_info.known_frame.clone());

        let mut seg_logic_header = LogicSimHeaderEx::start(welcome_info.known_frame.clone());
        let seg_graphical = GraphicalEx::start(seg_logic_header.calculated_heads.take().unwrap(), welcome_info.assigned_player_id);
        let seg_input_handler = InputHandler::new(
             welcome_info.assigned_player_id,
             seg_graphical.input_rec,
             self.seg_connect_net.net_sink.clone()
            );
        ClientEx{
            seg_input_handler,
            seg_scheduler,
            seg_data_storage,
            seg_logic_tailer,
            seg_logic_header,
        }
    }

}
struct ClientEx{
    seg_scheduler: SchedulerSegEx,
    seg_data_storage: SimDataStorage,
    seg_logic_tailer: LogicSimTailer,
    seg_logic_header: LogicSimHeaderEx,
    seg_input_handler: InputHandler
}
impl ClientEx{
    fn update_net_rec(&mut self, connected_client : &mut ConnectedClient){
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
                    self.seg_logic_tailer.check_hash(framed_hash);
                },
                _ => {
                    panic!("Client shouldn't be getting a message of this type (or at this time)!")
                }
            }
        }
    }
    fn post_interesting(mut self, mut connected_client: ConnectedClient, my_init_frame: FrameIndex){
        let downloaded_msg = ExternalMsg::WorldDownloaded(WorldDownloadedInfo{
            player_name: connected_client.player_name.clone(),
            color: connected_client.color.clone()
        });
        connected_client.seg_connect_net.net_sink.send((downloaded_msg, true)).unwrap();

        let known_frame = connected_client.welcome_info.known_frame.clone();

        let time_syncer = known_frame.start_frame_stream_from_now();
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


    }
}










