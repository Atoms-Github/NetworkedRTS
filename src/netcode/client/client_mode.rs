use crossbeam_channel::*;
use std::thread;
use std::time::{Duration};

use crate::netcode::client::connect_net_seg::*;
use crate::netcode::client::graphical_seg::*;
use crate::netcode::client::logic_sim_header_seg::*;
use crate::netcode::common::logic::logic_sim_tailer_seg::*;
use crate::netcode::common::network::external_msg::*;
use crate::netcode::common::sim_data::input_state::*;
use crate::netcode::common::sim_data::confirmed_data::*;
use crate::netcode::common::time::scheduler_segment::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::common::sim_data::superstore_seg::*;
use crate::netcode::client::input_handler_seg::*;
use ggez::input::keyboard::KeyCode;
use crate::pub_types::{FrameIndex, PlayerID, Shade, HashType};
use ggez::input::gamepad::gamepad;
use crate::rts::GameState;
use std::sync::Arc;
use crate::netcode::client::client_data_store::ClientDataStore;
use std::collections::HashMap;
use crate::netcode::common::simulation::logic_sim_tailer_seg::LogicSimTailer;
use crate::netcode::common::simulation::logic_sim_header_seg::{HeadSimPacket, HEAD_AHEAD_FRAME_COUNT};
use crate::netcode::common::simulation::net_game_state::NetGameState;


struct Client {
    player_id: PlayerID,
    net: ClientNet,
    player_name: String,
    color: Shade,
    data: ClientDataStore,
    hashes: HashMap<FrameIndex, HashType>,
    game_state: NetGameState,
    head_handle: Sender<HeadSimPacket>,
    curret_input: InputState,
    known_frame: KnownFrameInfo,
}
impl Client {
    pub fn go(player_name: String, color: Shade, connection_ip: String, preferred_port: i32){
        log::info!("Starting as client.");

        let mut net = ClientNet::start(connection_ip.clone());
        let mut welcome_info = net.get_synced_greeting();
        log::info!("Downloaded game state which has simmed {}", welcome_info.game_state.get_simmed_frame_index());

        net.client.send_msg(ExternalMsg::WorldDownloaded { player_name: player_name.clone(), color }, true);

        let mut data = ClientDataStore::new();

        // Init storage for all existing players. (they won't get inited by a ServerEvent.)
        data.glean_connected_players(&welcome_info.game_state);

        let (tx_head, rx_head) = unbounded();
        let (tx_input, rx_input) = unbounded();

        let known_frame = welcome_info.known_frame.clone();
        let mut client = Client{
            player_id: welcome_info.assigned_player_id,
            net,
            player_name,
            color,
            data,
            hashes: Default::default(),
            game_state: welcome_info.game_state,
            head_handle: tx_head,
            curret_input: Default::default(),
            known_frame
        };
        thread::spawn(move ||{
            client.core_loop(rx_input);
        });
        LogicSimHeaderEx::start_loop(rx_head);
    }
    fn on_new_head_frame(&mut self, head_frame: FrameIndex){
        let old_tail_frame = self.game_state.get_simmed_frame_index();

        let issue = self.game_state.sim_far_as_pos(&self.data.confirmed_data);
        let new_tail_frame = self.game_state.get_simmed_frame_index();
        let did_trail_progress = new_tail_frame != old_tail_frame;

        // I.e. we'll request data if we can't even make it to here.
        // The lower this is, the more work we're happy to do before complaining.
        let intended_tail = head_frame - HEAD_AHEAD_FRAME_COUNT;
        if new_tail_frame < intended_tail{
            self.net.client.send_msg(ExternalMsg::InputQuery(issue), false);
        }

        if did_trail_progress{
            self.send_head_state(self.game_state.clone());
        }

    }
    pub fn send_head_state(&mut self, gamestate: NetGameState){
        let sim_data = self.get_head_sim_data(data_store, gamestate.get_simmed_frame_index() + 1);
        let head_packet = HeadSimPacket{
            game_state: gamestate,
            sim_data
        };
        self.head_handle.send(head_packet).unwrap();
    }
    fn on_new_net_msg(&mut self, message: ExternalMsg){
        match message{
            ExternalMsg::GameUpdate(update) => {
                if crate::DEBUG_MSGS_MAIN {
                    log::debug!("Net rec message: {:?}", update);
                }
                // log::info!("Client Leart: {:?}", update);
                self.seg_data_storage.write_data(update);
            },
            ExternalMsg::InputQuery(query) => {
                let owned_data = self.seg_data_storage.fulfill_query(&query, 20);
                if owned_data.get_size() == 0{
                    log::info!("Failed to fulfil query {:?}", query);
                }else{
                    log::info!("Responded to server req for {:?} with {:?} items", query, owned_data);
                    connected_client.seg_connect_net.net_sink.send((ExternalMsg::GameUpdate(owned_data), false)).unwrap();

                }
            },
            ExternalMsg::PingTestResponse(_) => {
                // Do nothing. Doesn't matter that intro stuff is still floating when we move on.
            }
            ExternalMsg::NewHash(framed_hash) => {
                self.seg_logic_tailer.add_hash(framed_hash);
            },
            _ => {
                panic!("Client shouldn't be getting a message of this type (or at this time)!")
            }
        }
    }
    fn on_new_input(&mut self, input: InputChange){
        self.apply_input_changes();

        if let Some(my_next_empty) = data_store.get_next_empty(self.player_id) {
            let mut my_inputs_vec = vec![];
            for abs_frame_index in my_next_empty..(inputs_arriving_for_frame + 1) {
                my_inputs_vec.push(self.curret_input.clone());
            }
            data_store.write_input_data_single(self.player_id, self.curret_input.clone(), inputs_arriving_for_frame);

            let data_package = SimDataPackage::PlayerInputs(SuperstoreData {
                data: my_inputs_vec,
                frame_offset: my_next_empty
            }, self.player_id);
            //println!("Self input for frame: {} till {} excl second", my_next_empty, inputs_arriving_for_frame);
            data_store.write_data(data_package.clone());
            self.to_net.send((ExternalMsg::GameUpdate(data_package), false)).unwrap();
        }
    }
    fn apply_input_changes(&mut self){
        loop{
            let mut next_input = self.inputs_stream.try_recv();
            match next_input{
                Ok(input_change) => {
                    input_change.apply_to_state(&mut self.curret_input);
                }
                Err(e) => {
                    return;
                }
            }
        }
    }
    pub fn core_loop(mut self, inputs: Receiver<InputChange>){
        let head_frames = self.known_frame.start_frame_stream_from_now();
        loop{
            crossbeam_channel::select! {
                recv(self.net.client.up) -> msg ==>{
                    self.on_new_net_msg(msg);
                },
                recv(head_frames) -> new_frame ==>{
                    self.on_new_head_frame(new_frame);
                },
                recv(inputs) -> new_input ==>{
                    self.on_new_input(new_input);
                }
            }
        }
    }

}








