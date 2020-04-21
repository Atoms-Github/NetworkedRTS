use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{SystemTime};

use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_hub_segment::*;
use crate::network::networking_structs::*;
use crate::network::networking_message_types::{NetMessageType, NetMsgConnectionInitResponse};
use std::sync::{Mutex, Arc, RwLock};
use std::thread;
use std::panic;
use crate::game::bonus_msgs_segment::*;
use crate::game::logic::logic_data_storage::*;
use crate::game::logic::logic_segment::*;
use crate::game::synced_data_stream::*;
use crate::game::logic::data_storage_manager::*;


pub enum ServerActableMessage{
    NewlyGeneratedBonusMsgs(SyncerData<Vec<BonusEvent>>),
    IncomingClientMsg(OwnedNetworkMessage),
}
struct ServerMainStateEx {
////    all_frames: LogicDataStorage,
//    big_fat_zero_time: KnownFrameInfo,
//    outgoing_client_messages: Sender<DistributableNetMessage>,
//    all_incoming_messages: Receiver<ServerActableMessage>,
//    game_state_tail: Arc<Mutex<GameState>>,
//    logic_updates_sink: Sender<LogicInwardsMessage>,
//    logic_updates_rec: Receiver<LogicOutwardsMessage>,
//    bonus_scheduler_sink: Sender<BonusEvent>,
//    game_data_store: DataStorageManagerIn
    seg_net_hub: NetworkingHubEx,
    seg_data_store: DataStorageManagerEx,
    seg_logic_tail: LogicSegmentTailerEx,
    seg_bonus_msgs: BonusMsgsSegmentEx,
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
            known_frame: KnownFrameInfo{
                known_frame_index: 0,
                time: SystemTime::now()
            },
            hosting_ip
        }
    }
    fn init_state(&self) -> GameState{
        let mut game_state = GameState::new();
        game_state.init_rts();
        return game_state;
    }
    fn init_storage_man(&self) -> DataStorageManagerEx{
        let storage = LogicDataStorage::new(0);
        let data_store_setup = DataStorageManagerIn::new(storage);
        return data_store_setup.init_data_storage();
    }
    fn init_network_hub(&self) -> NetworkingHubEx{
        let net_hub_setup = NetworkingHubIn::new(self.hosting_ip.clone());
        return net_hub_setup.start_hosting();
    }
    fn init_logic_tailer(&self, data_handle: Arc<RwLock<LogicDataStorage>>) -> LogicSegmentTailerEx{
        let game_state = self.init_state();
        let setup = LogicSegmentTailerIn::new(self.known_frame.clone(), game_state, data_handle);
        return setup.start_logic_tail();
    }
    pub fn start_segments(self) -> ServerMainStateEx {
        let seg_net_hub = self.init_network_hub();
        let seg_data_store = self.init_storage_man();
        let seg_logic_tail = self.init_logic_tailer(seg_data_store.clone_lock_ref());
        let seg_bonus_msgs = BonusMsgsSegmentIn::new(self.known_frame.clone()).start();
        return ServerMainStateEx {
            seg_net_hub,
            seg_data_store,
            seg_logic_tail,
            seg_bonus_msgs,
            known_frame_zero: self.known_frame,
        }
    }

}
impl ServerMainStateEx {
    pub fn merge_server_actable_msgs(&mut self)
                                     -> Receiver<ServerActableMessage>{
        let inc_clients = self.seg_net_hub.pickup_rec.take().unwrap();
        let inc_bonus_msgs = self.seg_bonus_msgs.scheduled_events.take().unwrap();

        let (actable_sink,actable_rec) = channel();

        let actable_from_clients = actable_sink.clone();
        thread::spawn(move ||{
            loop{
                let client_message = inc_clients.recv().unwrap();
                actable_from_clients.send(ServerActableMessage::IncomingClientMsg(client_message)).unwrap();
            }
        });
        let actable_from_clients = actable_sink.clone();
        thread::spawn(move ||{
            loop{
                let bonus_msg = inc_bonus_msgs.recv().unwrap();
                actable_from_clients.send(ServerActableMessage::NewlyGeneratedBonusMsgs(bonus_msg)).unwrap();
            }
        });
        return actable_rec;
    }
    pub fn startup_loop(mut self){
        let server_actable_msgs = self.merge_server_actable_msgs();
//        let clients_rec = self.seg_net_hub.pickup_rec.take().unwrap();
//        let server_actable_msgs =
//            self.merge_server_actable_msgs(clients_rec,
//                                           self.seg_bonus_msgs.scheduled_events.take().unwrap());

        loop{
            let incoming_actable_message = server_actable_msgs.recv().unwrap();
            match incoming_actable_message{
                ServerActableMessage::IncomingClientMsg(incoming_owned_message) => {
                    self.handle_incoming_client_msg(incoming_owned_message);
                }
                ServerActableMessage::NewlyGeneratedBonusMsgs(new_bonus_msg) => {
                    self.handle_new_bonus_event(new_bonus_msg)
                }
            }

        }
    }
    fn gen_init_info(&self, player_id: PlayerID, frame_to_init: FrameIndex) -> NetMsgConnectionInitResponse{
        let state_to_send = self.seg_logic_tail.tail_lock.read().unwrap().clone(); // pointless_optimum this shouldn't need to be cloned to be serialized.
        let frames_to_send;
        {
            frames_to_send = self.seg_data_store.data_lock.read().unwrap().clone();
        }
        return NetMsgConnectionInitResponse{
            assigned_player_id: player_id,
            frames_gathered_so_far: frames_to_send,
            known_frame_info: self.known_frame_zero.clone(),
            game_state: state_to_send,
            you_initialize_frame: frame_to_init
        };
    }

    fn handle_incoming_client_msg(&mut self, incoming_owned_message: OwnedNetworkMessage){
        let incoming_message = incoming_owned_message.message;
        let player_id = incoming_owned_message.owner;
        match incoming_message{
            NetMessageType::ConnectionInitQuery(response) => {
                println!("Received initialization request for player with ID: {}", player_id);

                let frame_to_init_player = self.known_frame_zero.get_intended_current_frame() + 60;
                self.seg_bonus_msgs.schedule_event_timed(BonusEvent::NewPlayer(player_id), frame_to_init_player);
                let response = NetMessageType::ConnectionInitResponse(self.gen_init_info(player_id, frame_to_init_player));
                self.seg_net_hub.yeet_sink.send(DistributableNetMessage::ToSingle(player_id, response)).unwrap();
            },
            NetMessageType::GameUpdate(update_info) => {
                self.seg_data_store.logic_msgs_sink.send(update_info).unwrap();
            },
            _ => {
                panic!("Unexpected message");
            }
        }
    }
    fn handle_new_bonus_event(&mut self, new_bonus_msg: SyncerData<Vec<BonusEvent>>) -> () {
        // Send to all clients + self logic.
        let logic_update = LogicInwardsMessage::SyncerBonusUpdate(new_bonus_msg);
        println!("Sending {:?}", logic_update);
        self.seg_data_store.logic_msgs_sink.send(logic_update.clone()).unwrap();
        self.seg_net_hub.yeet_sink.send(DistributableNetMessage::ToAll(NetMessageType::GameUpdate(logic_update))).unwrap();
    }
}








