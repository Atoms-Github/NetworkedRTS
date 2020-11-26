use crossbeam_channel::*;
use std::thread;
use std::time::{Duration};

use crate::client::connect_net_seg::*;
use crate::client::graphical_seg::*;
use crate::client::logic_sim_header_seg::*;
use crate::client::net_dist_inputs_seg::*;
use crate::common::logic::logic_sim_tailer_seg::*;
use crate::common::network::external_msg::*;
use crate::common::sim_data::input_state::*;
use crate::common::sim_data::sim_data_storage::*;
use crate::common::data::hash_seg::*;
use crate::common::time::scheduler_segment::*;
use crate::client::net_rec_seg::*;
use crate::common::time::timekeeping::*;
use crate::common::types::*;
use crate::common::sim_data::framed_vec::*;
use crate::common::sim_data::superstore_seg::*;
use crate::client::input_handler_seg::*;
use ggez::input::keyboard::KeyCode;

struct ClientIn{
    player_name: String,
    connection_ip: String,
    preferred_id: i32,
}
// TODO2: Also, that refactor seems good. To prevent threads just repassing messages, the message generator should also be formatted for new version.
impl ClientIn{
    // Links up channels.
    fn init(self) -> ClientEx{
        let mut seg_net_connect = ConnectNetIn::new(self.connection_ip.clone()).start_net();
        let mut welcome_info = seg_net_connect.receive_synced_greeting(&self.player_name, self.preferred_id);


        let seg_data_storage = SimDataStorageEx::new(welcome_info.players_in_state, welcome_info.game_state.get_simmed_frame_index() + 1);

        let seg_hasher = HasherIn::start();
        let seg_scheduler = SchedulerSegIn::new(welcome_info.known_frame.clone()).start();
        let mut seg_logic_tailer = LogicSegmentTailerIn::new(welcome_info.known_frame.clone(), welcome_info.game_state, seg_data_storage.clone()).start_logic_tail();

        // Send local logic hashes.
        seg_hasher.link_hash_stream(seg_logic_tailer.new_tail_hashes.take().unwrap());
        let mut seg_logic_header = LogicSimHeaderIn::new(welcome_info.known_frame.clone(), seg_logic_tailer.new_tail_states_rec.take().unwrap(), seg_data_storage.clone()).start();
        let input_changes = GraphicalSeg::new(seg_logic_header.head_rec.take().unwrap(), welcome_info.assigned_player_id).start();


        let seg_net_rec = NetRecSegIn::new(seg_data_storage.clone(), seg_net_connect.net_rec.take().unwrap(), welcome_info.known_frame.clone(), seg_hasher.clone()).start();


        ClientEx{
            seg_net_connect,
            seg_scheduler,
            seg_data_storage,
            seg_logic_tailer,
            seg_logic_header,
            seg_hasher,
            input_changes,
            player_id: welcome_info.assigned_player_id,
            known_frame: welcome_info.known_frame,
        }
    }
}
struct ClientEx{
    seg_net_connect: ConnectNetEx,
    seg_scheduler: SchedulerSegEx,
    seg_data_storage: SimDataStorageEx,
    seg_logic_tailer: LogicSimTailerEx,
    seg_logic_header: LogicSimHeaderEx,
    seg_hasher: HasherEx,
    input_changes: Receiver<InputChange>,
    player_id: PlayerID,
    known_frame: KnownFrameInfo,
}

impl ClientEx{
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

    // InterestingClientLogic.
    fn run_loop(self){

        let my_init_frame = self.known_frame.get_intended_current_frame() + 50; // modival How far in the future to plonk yourself.

        println!("I'm gonna init me on {}", my_init_frame);
        let init_me_msg = self.gen_init_me_msgs(my_init_frame, self.player_id);
        self.seg_net_connect.net_sink.send((ExternalMsg::GameUpdate(init_me_msg.clone()),true)).unwrap();
        self.seg_data_storage.write_owned_data(init_me_msg);

        let seg_input_dist = InputHandlerIn::new
            (self.known_frame.clone(),
             self.player_id,
             my_init_frame + 1 /*Don't want to override existing frame which has 'NewPlayer' = true.*/,
             self.input_changes,
             self.seg_data_storage.clone()
        ).start();

        let seg_net_dist = NetInputDistIn::new(self.known_frame.clone(), self.player_id, self.seg_net_connect.net_sink.clone(), self.seg_data_storage.clone()).start_net_dist();

        loop{
            thread::sleep(Duration::from_millis(10000));
        }
    }

}




pub fn client_main(connection_target_ip: String, prefered_id: i32){
    thread::sleep(Duration::from_millis(1000)); // TODO1
    println!("Starting as client.");
    let client = ClientIn{
        player_name: String::from("Atomserdiah"),
        connection_ip: connection_target_ip,
        preferred_id: prefered_id
    };
    let client_ex = client.init();
    client_ex.run_loop();
}





























