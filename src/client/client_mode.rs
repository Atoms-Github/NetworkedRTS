use crossbeam_channel::*;
use std::thread;
use std::time::{Duration};

use crate::client::connect_net_seg::*;
use crate::client::graphical_seg::*;
use crate::client::logic_sim_header_seg::*;
use crate::client::net_dist_inputs_seg::*;
use crate::common::logic::logic_sim_tailer_seg::LogicSimTailerEx;
use crate::common::network::external_msg::*;
use crate::common::sim_data::input_state::*;
use crate::common::sim_data::sim_data_storage::*;
use crate::common::data::hash_seg::*;
use crate::common::time::scheduler_segment::*;
use crate::common::time::timekeeping::*;
use crate::common::types::*;
use crate::common::sim_data::framed_vec::*;
use crate::common::sim_data::superstore_seg::*;
use crate::client::input_handler_seg::*;
use ggez::input::keyboard::KeyCode;
pub struct ClientApp{
    player_name: String,
    connection_ip: String,
    preferred_id: i32,
}
impl ClientApp{
    pub fn go(player_name: String, connection_ip: String, preferred_id: i32){
        // Steps are:
        // 1. Init connection and download world.
        // 2. Do pre interesting.
        // 3. Create segs, and pass in init frame.
        // 4. Do post interesting.
        //
        // 1 is executed by the app and results in a connected client.
        // 2 and 3 are executed by the connected client and result in a ClientEx.
        // 4 is executed on the ClientEx.
        //
        println!("Starting as client.");
        let mut app = ClientApp{
            player_name,
            connection_ip,
            preferred_id
        };
        let connected_client = app.init_connection();
        connected_client.start();
    }
    fn init_connection(self) -> ConnectedClient{

        let mut seg_connect_net = ConnectNetEx::start(self.connection_ip.clone());

        let my_details = NetMsgGreetingQuery {
            my_player_name: self.player_name.to_string(),
            preferred_id: 5,
            udp_port: seg_connect_net.udp_port,
        };


        let mut welcome_info = seg_connect_net.get_synced_greeting(my_details);

        return ConnectedClient{
            welcome_info,
            seg_connect_net,
        }
    }
}
struct ConnectedClient{
    welcome_info: NetMsgGreetingResponse,
    seg_connect_net: ConnectNetEx,
}
impl ConnectedClient{
    pub fn start(mut self){
        let my_init_frame = self.pre_interesting();
        let ex = self.init_segs(my_init_frame);
        ex.post_interesting(self, my_init_frame);
    }
    fn pre_interesting(&self) -> FrameIndex{
        let my_init_frame = self.welcome_info.known_frame.get_intended_current_frame() + 50; // modival How far in the future to plonk yourself.
        println!("I'm gonna init me on {}", my_init_frame);
        return my_init_frame;
    }
    fn init_segs(&mut self, my_init_frame: FrameIndex) -> ClientEx{
        let welcome_info = self.welcome_info.clone();

        let seg_data_storage = SimDataStorageEx::new(welcome_info.players_in_state, welcome_info.game_state.get_simmed_frame_index() + 1);
        let seg_hasher = HasherEx::start();
        let seg_scheduler = SchedulerSegEx::start(welcome_info.known_frame.clone());
        let mut seg_logic_tailer = LogicSimTailerEx::start(welcome_info.known_frame.clone(), welcome_info.game_state, seg_data_storage.clone());

        // Send local logic hashes. TODO2: move to interesting?
        seg_hasher.link_hash_stream(seg_logic_tailer.new_tail_hashes.take().unwrap());
        let mut seg_logic_header = LogicSimHeaderEx::start(welcome_info.known_frame.clone(), seg_logic_tailer.new_tail_states_rec.take().unwrap(), seg_data_storage.clone());
        let seg_graphical = GraphicalEx::start(seg_logic_header.head_rec.take().unwrap(), welcome_info.assigned_player_id);
        let seg_input_dist = InputHandlerEx::start(
            welcome_info.known_frame.clone(),
             welcome_info.assigned_player_id,
             my_init_frame + 1 /*Don't want to override existing frame which has 'NewPlayer' = true.*/,
             seg_graphical.input_rec,
             seg_data_storage.clone()
            );
        let seg_net_dist = NetInputDistEx::start(welcome_info.known_frame.clone(), welcome_info.assigned_player_id,
                                               self.seg_connect_net.net_sink.clone(), seg_data_storage.clone());
        ClientEx{
            seg_scheduler,
            seg_data_storage,
            seg_logic_tailer,
            seg_logic_header,
            seg_hasher
        }
    }

}
struct ClientEx{
    seg_scheduler: SchedulerSegEx,
    seg_data_storage: SimDataStorageEx,
    seg_logic_tailer: LogicSimTailerEx,
    seg_logic_header: LogicSimHeaderEx,
    seg_hasher: HasherEx,
}
impl ClientEx{
    fn post_interesting(self, connected_client: ConnectedClient, my_init_frame: FrameIndex){
        let init_me_msg = self.gen_init_me_msgs(my_init_frame, connected_client.welcome_info.assigned_player_id);
        connected_client.seg_connect_net.net_sink.send((ExternalMsg::GameUpdate(init_me_msg.clone()),true)).unwrap();
        self.seg_data_storage.write_owned_data(init_me_msg);




        let inc_msgs = connected_client.seg_connect_net.net_rec.unwrap();
        loop{
            match inc_msgs.recv().unwrap(){
                ExternalMsg::GameUpdate(update) => {
                    if crate::DEBUG_MSGS_MAIN {
                        println!("Net rec message: {:?}", update);
                    }

                    self.seg_data_storage.write_owned_data(update);
                },
                ExternalMsg::PingTestResponse(_) => {
                    // Do nothing. Doesn't matter that intro stuff is still floating when we move on.
                }
                ExternalMsg::NewHash(framed_hash) => {
                    self.seg_hasher.add_hash(framed_hash);
                },
                _ => {
                    panic!("Client shouldn't be getting a message of this type (or at this time)!")
                }
            }
        }
    }
    fn gen_init_me_msgs(&self, frame_to_init_on: FrameIndex, my_player_id: PlayerID) -> OwnedSimData{
        let mut first_input = InputState::new();
        first_input.new_player = true;
        OwnedSimData{
            player_id: my_player_id,
            sim_data: SuperstoreData {
                data: vec![first_input],
                frame_offset: frame_to_init_on,
            }
        }
    }
}

















