use std::panic;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::SystemTime;

use crate::common::gameplay::game::game_state::*;
use crate::common::logic::logic_sim_tailer_seg::*;
use crate::common::network::external_msg::*;
use crate::common::sim_data::sim_data_storage::*;
use crate::common::sim_data::sim_data_storage_manager::*;
use crate::common::time::timekeeping::*;
use crate::common::types::*;
use crate::server::networking_hub_seg::*;

pub enum ServerActableMessage{
    IncomingClientMsg(OwnedNetworkMessage),
}
pub struct ServerMainStateEx {
    seg_net_hub: NetworkingHubEx,
    seg_data_store: SimDataStorageManagerEx,
    seg_logic_tail: LogicSimTailerEx,
    known_frame_zero: KnownFrameInfo
}

pub fn server_main(hosting_ip: String){
    println!("Starting as server. Going to host on {}", hosting_ip);

    let server_in = ServerMainStateIn::new(hosting_ip);
    let server_ex = server_in.start_segments();
    server_ex.startup_loop();

    println!("Server finished.");
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
        return game_state;
    }
    fn init_storage_man(&self) -> SimDataStorageManagerEx{
        let data_store_setup = SimDataStorageManagerIn::new(0);
        return data_store_setup.init_data_storage();
    }
    fn init_network_hub(&self) -> NetworkingHubEx{
        let net_hub_setup = NetworkingHubIn::new(self.hosting_ip.clone());
        return net_hub_setup.start_hosting();
    }
    fn init_logic_tailer(&self, data_handle: Arc<RwLock<SimDataStorage>>) -> LogicSimTailerEx{
        let game_state = self.init_state();
        let setup = LogicSegmentTailerIn::new(self.known_frame.clone(), game_state, data_handle);
        return setup.start_logic_tail();
    }
    pub fn start_segments(self) -> ServerMainStateEx {
        let seg_net_hub = self.init_network_hub();
        let seg_data_store = self.init_storage_man();
        let seg_logic_tail = self.init_logic_tailer(seg_data_store.clone_lock_ref());
        return ServerMainStateEx {
            seg_net_hub,
            seg_data_store,
            seg_logic_tail,
            known_frame_zero: self.known_frame,
        }
    }

}
impl ServerMainStateEx {
    pub fn merge_server_actable_msgs(&mut self)
                                     -> Receiver<ServerActableMessage>{
        let inc_clients = self.seg_net_hub.pickup_rec.take().unwrap();

        let (actable_sink,actable_rec) = channel();

        let actable_from_clients = actable_sink.clone();
        thread::spawn(move ||{
            loop{
                let client_message = inc_clients.recv().unwrap();
                actable_from_clients.send(ServerActableMessage::IncomingClientMsg(client_message)).unwrap();
            }
        });
        return actable_rec;
    }
    pub fn startup_loop(mut self){
        let server_actable_msgs = self.merge_server_actable_msgs();
        loop{
            let incoming_actable_message = server_actable_msgs.recv().unwrap();
            match incoming_actable_message{
                ServerActableMessage::IncomingClientMsg(incoming_owned_message) => {
                    self.handle_incoming_client_msg(incoming_owned_message);
                }
            }

        }
    }
    fn gen_init_info(&self, player_id: PlayerID) -> NetMsgGreetingResponse {
        let state_to_send = self.seg_logic_tail.tail_lock.read().unwrap().clone(); // pointless_optimum this shouldn't need to be cloned to be serialized.

        return NetMsgGreetingResponse {
            assigned_player_id: player_id,
            known_frame: self.known_frame_zero.clone(),
            game_state: state_to_send,
        };
    }

    fn handle_incoming_client_msg(&mut self, incoming_owned_message: OwnedNetworkMessage){
        let incoming_message = incoming_owned_message.message;
        let player_id = incoming_owned_message.owner;
        match incoming_message{
            ExternalMsg::ConnectionInitQuery(response) => {
                println!("Received initialization request for player with ID: {}", player_id);
                let response = ExternalMsg::ConnectionInitResponse(self.gen_init_info(player_id));
                self.seg_net_hub.yeet_sink.send(DistributableNetMessage::ToSingle(player_id, response)).unwrap();
            },
            ExternalMsg::GameUpdate(update_info) => {
                self.seg_data_store.logic_msgs_sink.send(update_info.clone()).unwrap();
                self.seg_net_hub.yeet_sink.send(
                    DistributableNetMessage::ToAllExcept(player_id, ExternalMsg::GameUpdate(update_info))
                ).unwrap();
            },
            ExternalMsg::PingTestQuery(client_time) => {
                // Do nothing. This message arrived too late.
            }
            _ => {
                panic!("Unexpected message");
            }
        }
    }
}








