use std::sync::mpsc::{Sender};
use std::thread;

use crate::common::types::*;

use crate::client::logic_sim_header_seg::*;
use crate::common::network::external_msg::*;
use crate::common::time::timekeeping::*;
use crate::common::sim_data::sim_data_storage::*;

use std::sync::{Arc, RwLock};


pub struct NetInputDistEx {
}
impl NetInputDistEx {

}
#[derive(Debug)]
pub struct NetInputDistIn {
    known_frame: KnownFrameInfo,
    player_id: PlayerID,
    first_frame_to_send: FrameIndex,
    to_net: Sender<ExternalMsg>,
    sim_data_storage: SimDataStorageEx,
}
impl NetInputDistIn {
    // This segment's job is to send the player's last 20 inputs to the network.
    // We don't care if we get a very rare syncing issue where inputs come in after we send them off because we're going to be sending last 20
    // so it will correct the next time something is sent.
    pub fn new(known_frame: KnownFrameInfo, player_id: PlayerID, first_frame_to_send: FrameIndex,
                to_net: Sender<ExternalMsg>, sim_data_storage: SimDataStorageEx) -> NetInputDistIn {
        NetInputDistIn {
            known_frame,
            player_id,
            first_frame_to_send,
            to_net,
            sim_data_storage,
        }
    }

    pub fn start_dist(mut self) -> NetInputDistEx{
        let tail_frame_rec = self.known_frame.start_frame_stream_from_any(self.first_frame_to_send);
        thread::spawn(move ||{
            loop{
                let head_frame = tail_frame_rec.recv().unwrap() + HEAD_AHEAD_FRAME_COUNT;
//                let logic_message = LogicInwardsMessage::SyncerInputsUpdate(FramedVecDataPack{ TODO1
//                    data: vec![curret_input.clone()],
//                    start_frame: inputs_arriving_for_frame,
//                    owning_player: self.player_id,
//                });
//                self.to_net.send(ExternalMsg::GameUpdate(logic_message.clone())).unwrap();
//                self.to_logic.send(logic_message).unwrap(); // Even if there were no changes, still need to send.
//                inputs_arriving_for_frame = next_frame_index;
            }
        });


        NetInputDistEx{
        }
    }
}


