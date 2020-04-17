use std::net::SocketAddr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{SystemTime};

use crate::game::timekeeping::KnownFrameInfo;
use crate::network::networking_hub_segment::{DistributableNetMessage, NetworkingHub, OwnedNetworkMessage};
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
struct ServerMainState {
//    all_frames: LogicDataStorage,
    big_fat_zero_time: KnownFrameInfo,
    outgoing_client_messages: Sender<DistributableNetMessage>,
    all_incoming_messages: Receiver<ServerActableMessage>,
    game_state_tail: Arc<Mutex<GameState>>,
    logic_updates_sink: Sender<LogicInwardsMessage>,
    logic_updates_rec: Receiver<LogicOutwardsMessage>,
    bonus_scheduler_sink: Sender<BonusEvent>,
    game_data_store: DataStorageManager<LogicDataStorage>
}
pub fn gather_incoming_server_messages(inc_clients: Receiver<OwnedNetworkMessage>, bonus_msgs: Receiver<SyncerData<Vec<BonusEvent>>>)
                                       -> Receiver<ServerActableMessage>{
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
            let bonus_msg = bonus_msgs.recv().unwrap();
            actable_from_clients.send(ServerActableMessage::NewlyGeneratedBonusMsgs(bonus_msg)).unwrap();
        }
    });
    return actable_rec;
}

pub fn server_main(hosting_ip: &String){
    println!("Starting as server. Going to host on {}", hosting_ip);

    let server = ServerMainState::init_server_hosting(hosting_ip);
    server.server_logic_loop();

    println!("Server finished.");
}

impl ServerMainState{
    pub fn init_server_hosting(hosting_ip: &String) -> ServerMainState{
        // Init connection.
        let addr = hosting_ip.to_string().parse::<SocketAddr>().unwrap();
        let mut networking_hub_segment = NetworkingHub::new();
        let (mut outgoing_sender, outgoing_receiver) = channel();
        let incoming_client_messages =
            networking_hub_segment.start_listening(outgoing_receiver, addr);

        // Init logic.
        let big_fat_zero_time = KnownFrameInfo{
            known_frame_index: 0,
            time: SystemTime::now()
        };
        let mut game_state = GameState::new();
        game_state.init_rts();
        let (mut logic_outwards_sink, mut logic_outwards_rec) = channel();


        let storage_manager = DataStorageManager { value: Arc::new(RwLock::new(LogicDataStorage::new(0))) };


        let (mut logic_segment, mut state_handle) =
            LogicSegment::new(false, big_fat_zero_time.clone(),
                              game_state,logic_outwards_sink,
                              storage_manager.value.clone());

        let (mut game_updates_sink, mut game_updates_rec) = channel();
        thread::spawn(||{
            logic_segment.run_logic_loop(game_updates_rec);
        });

        let mut bonus_msgs_segment = BonusMsgsSegment::new(big_fat_zero_time.clone());
        let (new_bonus_msgs, bonus_scheduler_sink) = bonus_msgs_segment.start();

        let all_incoming_messages = gather_incoming_server_messages(incoming_client_messages, new_bonus_msgs);




        return ServerMainState{
            big_fat_zero_time,

            outgoing_client_messages: outgoing_sender,
            all_incoming_messages,
            game_state_tail: state_handle,
            logic_updates_sink: game_updates_sink,
            logic_updates_rec: logic_outwards_rec,
            bonus_scheduler_sink,
            game_data_store: storage_manager,
        }
    }

    fn handle_incoming_client_msg(&mut self, incoming_owned_message: OwnedNetworkMessage){
        let incoming_message = incoming_owned_message.message;
        let player_id = incoming_owned_message.owner;
        match incoming_message{
            NetMessageType::ConnectionInitQuery(response) => {

//                let (state_to_send, known_to_send) = self.get_accurate_state();
                let state_to_send = self.game_state_tail.lock().unwrap().clone(); // TODO3 this shouldn't need to be cloned to be serialized.
                let mut frames_to_send;
                {
                    let frames_guard = self.game_data_store.value.read().unwrap();
                    frames_to_send = frames_guard.clone();
                }


                println!("Going to send with this much info: {}", frames_to_send.bonus_events.data.len());
                let response = NetMessageType::ConnectionInitResponse(NetMsgConnectionInitResponse{
                    assigned_player_id: player_id,
                    frames_gathered_so_far: frames_to_send,
                    known_frame_info: self.big_fat_zero_time.clone(),
                    game_state: state_to_send,
                });
                println!("Received initialization request for player with ID: {}", player_id);
                self.outgoing_client_messages.send(DistributableNetMessage::ToSingle(player_id, response)).unwrap();
            },
            NetMessageType::GameUpdate(update_info) => {
                self.logic_updates_sink.send(update_info).unwrap();
            },
            _ => {
                panic!("Unexpected message");
            }
        }
    }
    fn handle_new_bonus_event(&mut self, new_bonus_msg: SyncerData<Vec<BonusEvent>>) -> () {
        // Send to all clients + self logic.
        let logic_update = LogicInwardsMessage::SyncerBonusUpdate(new_bonus_msg);
        self.logic_updates_sink.send(logic_update.clone()).unwrap();
        self.outgoing_client_messages.send(DistributableNetMessage::ToAll(NetMessageType::GameUpdate(logic_update.clone()))).unwrap();
        println!("Sending {:?}", logic_update);


    }
    pub fn server_logic_loop(mut self){
        loop{
            let incoming_actable_message = self.all_incoming_messages.recv().unwrap();
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
}








