use std::panic;
use std::sync::{Arc, RwLock};
use crossbeam_channel::*;
use std::thread;
use std::time::SystemTime;

use crate::common::gameplay::game::game_state::*;
use crate::common::logic::logic_sim_tailer_seg::*;
use crate::common::network::external_msg::*;
use crate::common::sim_data::sim_data_storage::*;
use crate::common::sim_data::sim_data_storage_manager::*;
use crate::common::time::timekeeping::*;
use crate::common::types::*;
use crate::server::net_hub_front_seg::*;

pub struct ServerMainStateEx {
    seg_net_hub: NetworkingHubEx,
    data_store: SimDataStorageEx,
    seg_logic_tail: LogicSimTailerEx,
    known_frame_zero: KnownFrameInfo
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
    fn init_state(&self) -> GameState{
        let mut game_state = GameState::new();
        game_state.init_rts();
        game_state
    }
    pub fn start_segments(self) -> ServerMainStateEx {
        let seg_net_hub = NetworkingHubEx::start(self.hosting_ip.clone());
        let seg_data_store = SimDataStorageEx::new(vec![], 0);
        let mut seg_logic_tail = LogicSimTailerEx::start(self.known_frame.clone(), self.init_state(), seg_data_store.clone());
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
        }
    }

}
impl ServerMainStateEx {
    pub fn main_loop(mut self){
        loop{
            match self.seg_net_hub.up_rec.recv().unwrap(){
                NetHubFrontMsgOut::NewPlayer(player_id) => {

                }
                NetHubFrontMsgOut::PlayerDiscon(player_id) => {

                }
                NetHubFrontMsgOut::NewMsg(msg, player_id) => {
                    match msg{
                        ExternalMsg::ConnectionInitQuery(response) => {
                            println!("Received initialization request for player with ID: {}", player_id);
                            let response = ExternalMsg::ConnectionInitResponse(self.gen_init_info(player_id));
                            self.seg_net_hub.down_sink.send(NetHubFrontMsgIn::MsgToSingle(response, player_id, true)).unwrap();
                        },
                        ExternalMsg::GameUpdate(update_info) => {
//                          println!("Recieved player {} inputs for frames {} to {} inclusive.", update_info.player_id, update_info.sim_data.frame_offset, update_info.sim_data.frame_offset + update_info.sim_data.data.len() - 1);
                            self.data_store.write_owned_data(update_info.clone());
                            self.seg_net_hub.down_sink.send(NetHubFrontMsgIn::MsgToAllExcept(ExternalMsg::GameUpdate(update_info),player_id, false)).unwrap();
                        },
                        ExternalMsg::InputQuery(query) => {
                            let owned_data = self.data_store.fulfill_query(&query);
                            if owned_data.sim_data.data.len() > 0{
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
    fn gen_init_info(&self, player_id: PlayerID) -> NetMsgGreetingResponse {
        let game_state = self.seg_logic_tail.tail_lock.read().unwrap().clone(); // pointless_optimum this shouldn't need to be cloned to be serialized.

        let existing_players = self.data_store.get_player_list(game_state.get_simmed_frame_index());

        NetMsgGreetingResponse {
            assigned_player_id: player_id,
            known_frame: self.known_frame_zero.clone(),
            game_state,
            players_in_state: existing_players
        }
    }
}


pub fn server_main(hosting_ip: String){
    println!("Starting as server. Going to host on {}", hosting_ip);

    let server_in = ServerMainStateIn::new(hosting_ip);
    let server_ex = server_in.start_segments();
    server_ex.main_loop();

    println!("Server finished.");
}







