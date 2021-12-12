use std::panic;
use std::sync::{Arc, RwLock};
use crossbeam_channel::*;
use std::thread;
use std::time::{SystemTime, Duration};
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use crate::netcode::*;
use crate::netcode::common::logic::logic_sim_tailer_seg::*;
use crate::netcode::common::network::external_msg::*;
use crate::netcode::common::sim_data::confirmed_data::*;
use crate::netcode::common::time::timekeeping::*;

use crate::netcode::*;
use crate::netcode::common::sim_data::confirmed_data::SimDataOwner::Player;
use crate::netcode::common::sim_data::superstore_seg::SuperstoreData;
use crate::rts::GameState;
use crate::netcode::common::simulation::net_game_state::NetGameState;

pub struct Server {
    seg_net_hub: NetworkingHubEx,
    data_store: ConfirmedData,
    seg_logic_tail: LogicSimTailer,
    known_frame_zero: KnownFrameInfo,
}


impl Server {
    pub fn start(hosting_ip: String){
        let seg_net_hub = bibble_tokio::start_server(hosting_ip.clone());
        let game_state = NetGameState::new();

        let event_distributor = ServerEventDistributor::new(seg_net_hub.down_sink.clone());

        let mut server = ServerMainStateEx {
            seg_net_hub,
            data_store: ConfirmedData::new(),
            seg_logic_tail,
            known_frame_zero: self.known_frame,
            missing_data_handler: SeverMissingDataHandler::new(missing_data_kick_msg_tx),
            event_distributor
        };
        server.main_loop();
    }
    fn main_loop(mut self){
        let frame_timer = self.known_frame_zero.start_frame_stream_from_now();
        loop{
            let current_sim_frame = frame_timer.recv().unwrap();

            while let Ok(net_event) = self.seg_net_hub.up_rec.try_recv(){
                self.handle_net_msg(net_event);
            }

            if let Some(missing_datas) = self.seg_logic_tail.catchup_simulation(&self.data_store, current_sim_frame){
                self.missing_data_handler.handle_requests(missing_datas);
            }
            self.distrubute_state_hash();

        }
    }
    fn distrubute_state_hash(&mut self){
        let game_state = &self.seg_logic_tail.game_state;
        self.seg_net_hub.down_sink.send(NetHubFrontMsgIn::MsgToAll(ExternalMsg::NewHash(FramedHash{
            frame: game_state.get_simmed_frame_index(),
            hash: game_state.get_hash()
        }), false)).unwrap();
    }
    fn on_new_net_msg(&mut self, message: ExternalMsg){

    }
    fn on_new_head_frame(&mut self, input: InputChange){
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
impl ServerMainStateEx {
    fn handle_net_msg(&mut self, net_event: NetHubFrontMsgOut){
        match net_event{
            NetHubFrontMsgOut::NewPlayer(player_id) => {}
            NetHubFrontMsgOut::PlayerDiscon(player_id) => {
                log::info!("Player disconnected! --------------------");
                self.data_store.server_boot_player(player_id, self.seg_logic_tail.game_state.get_simmed_frame_index());

            }
            NetHubFrontMsgOut::NewMsg(msg, player_id) => {
                match msg{
                    ExternalMsg::ConnectionInitQuery(greeting) => {
                        log::info!("Received initialization request for player with ID: {}", player_id);

                        let game_state = self.seg_logic_tail.game_state.clone(); // pointless_optimum this shouldn't need to be cloned to be serialized.

                        let msg = NetMsgGreetingResponse {
                            assigned_player_id: player_id,
                            known_frame: self.known_frame_zero.clone(),
                            game_state,
                        };
                        let response = ExternalMsg::ConnectionInitResponse(msg);
                        self.seg_net_hub.down_sink.send(NetHubFrontMsgIn::MsgToSingle(response, player_id, true)).unwrap();
                    },
                    ExternalMsg::WorldDownloaded(downloaded_info) => {
                        self.data_store.schedule_server_event(ServerEvent::JoinPlayer(player_id, downloaded_info.player_name, downloaded_info.color));
                    },
                    ExternalMsg::GameUpdate(update_info) => {
                        log::debug!("Server learned: {:?}", update_info);
                        self.data_store.write_data(update_info.clone());
                        self.seg_net_hub.down_sink.send(NetHubFrontMsgIn::MsgToAllExcept(ExternalMsg::GameUpdate(update_info),player_id, false)).unwrap();
                    },
                    ExternalMsg::InputQuery(query) => {
                        let owned_data = self.data_store.fulfill_query(&query, 20);
                        if owned_data.get_size() == 0{
                            log::info!("Failed to fulfil query {:?}", query);
                        }else{
                            log::debug!("Responded to {}'s req for {:?} with {:?} items", player_id, query, owned_data);
                            self.seg_net_hub.down_sink.send(NetHubFrontMsgIn::MsgToSingle(ExternalMsg::GameUpdate(owned_data),player_id, false)).unwrap();
                        }
                    },
                    _ => {
                        panic!("Unexpected message");
                    }
                }
            }
        }

    }


}


pub fn server_main(hosting_ip: String){
    log::info!("Starting as server. Going to host on {}", hosting_ip);

    let server_in = ServerMainStateIn::new(hosting_ip);
    let server_ex = server_in.start_segments();
    server_ex.main_loop();

    log::info!("Server finished.");
}
