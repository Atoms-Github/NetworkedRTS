use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration};

use crate::client::connect_net_seg::*;
use crate::client::graphical_seg::*;
use crate::client::logic_sim_header_seg::*;
use crate::common::logic::logic_sim_tailer_seg::*;
use crate::common::network::external_msg::*;
use crate::common::sim_data::input_state::*;
use crate::common::sim_data::sim_data_storage_manager::*;
use crate::common::time::scheduler_segment::*;
use crate::client::net_rec_seg::*;
use crate::common::time::timekeeping::*;
use crate::common::types::*;
use crate::common::sim_data::framed_vec::*;

struct ClientIn{
    player_name: String,
    connection_ip: String
}

impl ClientIn{
    fn init_net_rec_handling(&self, incoming_messages: Receiver<ExternalMsg>, to_logic: Sender<LogicInwardsMessage>){ // TODO1: Move to segment with gather.

    }

    // Links up channels.
    fn init(self) -> ClientEx{
        let mut seg_net = ConnectNetIn::new(self.connection_ip.clone()).start_net();
        let welcome_info = seg_net.receive_synced_greeting(&self.player_name);

        let seg_scheduler = SchedulerSegIn::new(welcome_info.known_frame.clone()).start();
        let seg_data_storage = SimDataStorageManagerIn::new(welcome_info.game_state.get_simmed_frame_index()).init_data_storage();
        let mut seg_logic_tailer = LogicSegmentTailerIn::new(welcome_info.known_frame.clone(), welcome_info.game_state, seg_data_storage.clone_lock_ref()).start_logic_tail();
        let mut seg_logic_header = LogicSimHeaderIn::new(welcome_info.known_frame.clone(), seg_logic_tailer.new_tail_states_rec.take().unwrap(), seg_data_storage.clone_lock_ref()).start();
        let input_changes = GraphicalSeg::new(seg_logic_header.head_rec.take().unwrap(), welcome_info.assigned_player_id).start();

        let seg_net_rec = NetRecSegIn::new(seg_data_storage.logic_msgs_sink.clone(), seg_net.net_rec.take().unwrap(), welcome_info.known_frame.clone()).start();

        ClientEx{
            seg_net,
            seg_scheduler,
            seg_data_storage,
            seg_logic_tailer,
            seg_logic_header,
            input_changes,
            player_id: welcome_info.assigned_player_id,
            known_frame: welcome_info.known_frame,
        }
    }
}
struct ClientEx{
    seg_net: ConnectNetEx,
    seg_scheduler: SchedulerSegEx,
    seg_data_storage: SimDataStorageManagerEx,
    seg_logic_tailer: LogicSimTailerEx,
    seg_logic_header: LogicSimHeaderEx,
    input_changes: Receiver<InputChange>,
    player_id: PlayerID,
    known_frame: KnownFrameInfo,

}

impl ClientEx{
    fn gen_init_me_msgs(&self, frame_to_init_on: FrameIndex, my_player_id: PlayerID) -> LogicInwardsMessage{
        let mut first_input = InputState::new();
        first_input.new_player = true;
        let syncer_data = FramedVecDataPack{
            data: vec![first_input],
            start_frame: frame_to_init_on,
            owning_player: my_player_id
        };
        LogicInwardsMessage::SyncerInputsUpdate(syncer_data)
    }

    // InterestingClientLogic.
    fn run_loop(self){

        let my_init_frame = self.known_frame.get_intended_current_frame() + 50; // modival

        println!("I'm gonna init me on {}", my_init_frame);
        let init_me_msg = self.gen_init_me_msgs(my_init_frame, self.player_id);
        self.seg_net.net_sink.send(ExternalMsg::GameUpdate(init_me_msg.clone())).unwrap();
        self.seg_data_storage.logic_msgs_sink.send(init_me_msg).unwrap();

//        let seg_input_dist = InputHandlerIn::new TODO1
//            (self.known_frame,
//             self.player_id,
//             my_init_frame,
//             self.seg_data_storage.logic_msgs_sink.clone(),
//             self.seg_net.net_sink.clone(),
//             self.seg_data_storage.clone_lock_ref()
//        );
//
//        let input_changes = self.input_changes;
//        self.seg_scheduler.schedule_event(Box::new(move ||{
//            seg_input_dist.start_dist(input_changes);
//        }), my_init_frame);






        loop{
            thread::sleep(Duration::from_millis(10000));
        }
    }

}




pub fn client_main(connection_target_ip: String){
    println!("Starting as client.");
    let client = ClientIn{
        player_name: String::from("Atomserdiah"),
        connection_ip: connection_target_ip,
    };
    let client_ex = client.init();
    client_ex.run_loop();
}





























