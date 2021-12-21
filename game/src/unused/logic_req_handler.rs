use std::time::SystemTime;
use crossbeam_channel::{Receiver, Sender};
use crate::netcode::common::sim_data::confirmed_data::{SimDataQuery, SimDataOwner};
use crate::netcode::server::net_hub_front_seg::NetHubFrontMsgIn;
use std::thread;
use crate::netcode::common::network::external_msg::ExternalMsg;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use std::ptr::read_unaligned;

// TODOops: Implement
pub struct SeverMissingDataHandler {
    waiting_on: PlayerID,
    waiting_since: SystemTime,
    is_waiting: bool,
    net_manager_tx: Sender<NetHubFrontMsgIn>,
}
impl SeverMissingDataHandler {
    pub fn handle_requests(&mut self, requests : Vec<SimDataQuery>){
        for request in requests{
            log::info!("Server missing {:?}", request);
            match request.query_type{
                SimDataOwner::Server => {
                    panic!("How can server be waiting for server events?");
                }
                SimDataOwner::Player(player_id) => {

                    self.net_manager_tx.send(NetHubFrontMsgIn::MsgToSingle(ExternalMsg::InputQuery(request), player_id, false)).unwrap();
                }
            }

        }
    }
    pub fn new(kick_msgs_tx: Sender<NetHubFrontMsgIn>) -> Self{
        SeverMissingDataHandler {
            waiting_on: 0,
            waiting_since: SystemTime::now(),
            is_waiting: false,
            net_manager_tx: kick_msgs_tx
        }
    }
}