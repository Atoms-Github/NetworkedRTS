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
    fn init_storage_man(&self) -> SimDataStorageEx{
        SimDataStorageEx::new(vec![], 0)
    }
    fn init_network_hub(&self) -> NetworkingHubEx{
        let net_hub_setup = NetworkingHubIn::new(self.hosting_ip.clone());
        net_hub_setup.start_hosting()
    }
    fn init_logic_tailer(&self, data_handle: SimDataStorageEx) -> LogicSimTailerEx{
        let game_state = self.init_state();
        LogicSimTailerEx::start(self.known_frame.clone(), game_state, data_handle)
    }
    pub fn start_segments(self) -> ServerMainStateEx {
        let seg_net_hub = self.init_network_hub();
        let seg_data_store = self.init_storage_man();
        let mut seg_logic_tail = self.init_logic_tailer(seg_data_store.clone());


        let hash_rec = seg_logic_tail.new_tail_hashes.take().unwrap(); // dans_game.
        let hash_net_yeet_sink = seg_net_hub.down_sink.clone();
        thread::spawn(move ||{
            loop{
                let framed_hash = hash_rec.recv().unwrap();
                hash_net_yeet_sink.send(NetHubFrontMsgIn::MsgToAll(ExternalMsg::NewHash(framed_hash), false)).unwrap();
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




