use crate::common::types::PlayerID;
use std::time::SystemTime;
use crossbeam_channel::{Receiver, Sender};
use crate::common::sim_data::sim_data_storage::QuerySimData;
use crate::server::net_hub_front_seg::NetHubFrontMsgIn;
use std::thread;
use crate::common::network::external_msg::ExternalMsg;

pub struct LogicReqHandlerIn {
    waiting_on: PlayerID,
    waiting_since: SystemTime,
    is_waiting: bool,
    server_logic_reqs_rc: Receiver<QuerySimData>,
    net_manager_tx: Sender<NetHubFrontMsgIn>,
}


impl LogicReqHandlerIn {
    fn start_thread(self){
        thread::spawn(move ||{
            loop{
                let request = self.server_logic_reqs_rc.recv().unwrap();
                let target = request.player_id;
                self.net_manager_tx.send(NetHubFrontMsgIn::MsgToSingle(ExternalMsg::InputQuery(request), target, false)).unwrap();
            }
        });
    }
}
pub struct LogicReqHandlerEx{

}
impl LogicReqHandlerEx{
    pub fn start(server_logic_reqs_rc: Receiver<QuerySimData>, kick_msgs_tx: Sender<NetHubFrontMsgIn>) -> Self{
        LogicReqHandlerIn{
            waiting_on: 0,
            waiting_since: SystemTime::now(),
            is_waiting: false,
            server_logic_reqs_rc,
            net_manager_tx: kick_msgs_tx
        }.start_thread();
        LogicReqHandlerEx{

        }
    }
}





