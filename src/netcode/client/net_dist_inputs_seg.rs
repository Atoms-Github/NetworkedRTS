use crossbeam_channel::*;
use std::thread;

use crate::netcode::client::logic_sim_header_seg::*;
use crate::netcode::common::network::external_msg::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::common::sim_data::sim_data_storage::*;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use std::sync::{Arc, RwLock};


pub struct NetInputDistEx {
}
impl NetInputDistEx {
    pub fn start(known_frame: KnownFrameInfo, player_id: PlayerID, to_net: Sender<(ExternalMsg,bool)>, sim_data_storage: SimDataStorage) -> NetInputDistEx {
        NetInputDistIn {
            known_frame,
            player_id,
            to_net,
            sim_data_storage,
        }.start_net_dist()
    }
}
#[derive()]
pub struct NetInputDistIn {
    known_frame: KnownFrameInfo,
    player_id: PlayerID,
    to_net: Sender<(ExternalMsg,bool)>,
    sim_data_storage: SimDataStorage,
}
impl NetInputDistIn {
    // This segment's job is to send the player's last 20 inputs to the network.
    // We don't care if we get a very rare syncing issue where inputs come in after we send them off because we're going to be sending last 20
    // so it will correct the next time something is sent.
    pub fn start_net_dist(mut self) -> NetInputDistEx{
        let gen_timekeeper = self.known_frame.start_frame_stream_from_now();
        thread::spawn(move ||{
            loop{
                let tail_frame = gen_timekeeper.recv().unwrap();

                let query = SimDataQuery {
                    frame_offset: tail_frame, // We don't care about any past data.
                    player_id: self.player_id
                };
                let my_inputs = self.sim_data_storage.fulfill_query(&query);
                log::trace!("Sending to net my inputs for frames {} to {} inclusive", my_inputs.input_data.frame_offset, my_inputs.input_data.frame_offset + my_inputs.input_data.data.len() - 1);
                self.to_net.send((ExternalMsg::GameUpdate(my_inputs),false)).unwrap();
            }
        });


        NetInputDistEx{
        }
    }
}


