use std::panic;
use std::sync::{Arc, RwLock};
use crossbeam_channel::*;
use std::thread;
use std::time::SystemTime;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use crate::netcode::*;
use crate::netcode::common::logic::logic_sim_tailer_seg::*;
use crate::netcode::common::network::external_msg::*;
use crate::netcode::common::sim_data::sim_data_storage::*;
use crate::netcode::common::time::timekeeping::*;

use crate::netcode::server::net_hub_front_seg::*;
use crate::netcode::*;
use crate::netcode::common::sim_data::net_game_state::{NetPlayerProperty, NetGameState};
use crate::netcode::common::sim_data::sim_data_storage::SimDataOwner::Player;
use crate::netcode::server::logic_req_handler::SeverMissingDataHandler;
use crate::netcode::common::sim_data::superstore_seg::SuperstoreData;

pub struct ServerMainStateEx {
    seg_net_hub: NetworkingHubEx,
    data_store: SimDataStorage,
    seg_logic_tail: LogicSimTailer,
    known_frame_zero: KnownFrameInfo,
    missing_data_handler: SeverMissingDataHandler,
}


pub struct ServerMainStateIn {
    known_frame: KnownFrameInfo,
    hosting_ip: String,
}
impl ServerMainStateIn {
    pub fn new(hosting_ip: String) -> ServerMainStateIn {
        ServerMainStateIn {
            known_frame: KnownFrameInfo::new_from_args(0, SystemTime::now()),
            hosting_ip
        }
    }
    fn init_state(&self) -> NetGameState {
        let mut game_state = NetGameState::new();
        game_state
    }
    pub fn start_segments(self) -> ServerMainStateEx {
        let seg_net_hub = NetworkingHubEx::start(self.hosting_ip.clone());
        let seg_data_store = SimDataStorage::new(0);
        let mut seg_logic_tail = LogicSimTailer::start(self.known_frame.clone(), self.init_state(), seg_data_store.clone());
        let hash_rec = seg_logic_tail.new_tail_hashes.take().unwrap(); // dans_game.
        let hash_net_tx = seg_net_hub.down_sink.clone();
        thread::spawn(move ||{
            loop{
                let framed_hash = hash_rec.recv().unwrap();
                //hash_net_tx.send(NetHubFrontMsgIn::MsgToAll(ExternalMsg::NewHash(framed_hash), false)).unwrap();
            }
        });

        ServerMainStateEx {
            seg_net_hub,
            data_store: seg_data_store,
            seg_logic_tail,
            known_frame_zero: self.known_frame,
            missing_data_handler: SeverMissingDataHandler::new(seg_net_hub.down_sink.clone()),
        }
    }

}
impl ServerMainStateEx {
    fn handle_net_msg(&mut self, net_event: NetHubFrontMsgOut){
        match net_event{
            NetHubFrontMsgOut::NewPlayer(player_id) => {}
            NetHubFrontMsgOut::PlayerDiscon(player_id) => {
                log::info!("Player disconnected! --------------------");
                // breaking Put in 'disconnect' server event, and insert blank inputs up to that point.
                self.data_store.schedule_server_event(ServerEvent::DisconnectPlayer(player_id));

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
                    ExternalMsg::WorldDownloaded() => {
                        self.data_store.schedule_server_event(ServerEvent::JoinPlayer(player_id));
                    },
                    ExternalMsg::GameUpdate(update_info) => {
                        //log::trace!("Recieved player {} inputs for frames {} to {} inclusive.", update_info.data_owner, update_info.input_data.frame_offset, update_info.input_data.frame_offset + update_info.input_data.data.len() - 1);
                        self.data_store.write_data(update_info.clone());
                        // Optimum Distribution should happen in net layer for fasttrax.
                        self.seg_net_hub.down_sink.send(NetHubFrontMsgIn::MsgToAllExcept(ExternalMsg::GameUpdate(update_info),player_id, false)).unwrap();
                    },
                    ExternalMsg::InputQuery(query) => {
                        let owned_data = self.data_store.fulfill_query(&query);
                        // optimum - don't send empty stuff.
                        self.seg_net_hub.down_sink.send(NetHubFrontMsgIn::MsgToSingle(ExternalMsg::GameUpdate(owned_data),player_id, false)).unwrap();
                    },
                    _ => {
                        panic!("Unexpected message");
                    }
                }
            }
        }
    }
    pub fn main_loop(mut self){
        let frame_timer = self.known_frame_zero.start_frame_stream_from_now();

        loop{
            let current_sim_frame = frame_timer.recv().unwrap();

            while let Ok(net_event) = self.seg_net_hub.up_rec.try_recv(){
                self.handle_net_msg(net_event);
            }
            if let Err(missing_datas) = self.seg_logic_tail.catchup_simulation(&self.data_store, current_sim_frame){
                self.missing_data_handler.handle_requests(missing_datas);
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
