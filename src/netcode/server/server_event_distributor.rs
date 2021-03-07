use crossbeam_channel::Sender;
use crate::netcode::server::net_hub_front_seg::NetHubFrontMsgIn;
use crate::netcode::common::sim_data::sim_data_storage::{SimDataStorage, SimDataPackage};
use crate::pub_types::FrameIndex;
use crate::netcode::client::logic_sim_header_seg::HEAD_AHEAD_FRAME_COUNT;
use crate::netcode::common::sim_data::superstore_seg::SuperstoreData;
use crate::netcode::common::network::external_msg::ExternalMsg;

pub struct ServerEventDistributor{
    to_net: Sender<NetHubFrontMsgIn>
}
impl ServerEventDistributor{
    pub fn new(to_net: Sender<NetHubFrontMsgIn>) -> Self{
        Self{
            to_net
        }
    }
    pub fn update(&mut self, data_store: &mut SimDataStorage, frame_tail_simed: FrameIndex) {
        let mut new_events = vec![];
        let next_non_existing_events = data_store.get_next_empty_server_events_frame();
        for frame_index in next_non_existing_events..(frame_tail_simed + HEAD_AHEAD_FRAME_COUNT + 1){
            new_events.push(vec![]);
        }

        println!("Server send server events from {} to {} inclusive both ends.", next_non_existing_events, new_events.len());
        let package = SimDataPackage::ServerEvents(SuperstoreData{
            data: new_events,
            frame_offset: next_non_existing_events
        });
        data_store.write_data(package.clone());
        self.to_net.send(NetHubFrontMsgIn::MsgToAll(ExternalMsg::GameUpdate(package), false  )).unwrap();

    }
}