use std::panic;
use crossbeam_channel::*;
use std::thread;
use std::time::{Duration};

use crate::common::logic::logic_sim_tailer_seg::*;
use crate::common::network::external_msg::*;
use crate::common::time::timekeeping::*;
use crate::common::sim_data::sim_data_storage::*;
use crate::common::data::hash_seg::HasherEx;


pub struct NetRecSegIn{
    incoming_msgs: Vec<i32>,
    net_inc: Receiver<ExternalMsg>,
    known_frame: KnownFrameInfo,
    storage: SimDataStorageEx,
    seg_hasher: HasherEx,
}
pub struct NetRecSegEx{

}
impl NetRecSegEx{
    pub fn start(storage: SimDataStorageEx, net_inc: Receiver<ExternalMsg>, known_frame: KnownFrameInfo, seg_hasher: HasherEx) -> Self{
        NetRecSegIn{
            incoming_msgs: vec![],
            net_inc,
            known_frame,
            storage,
            seg_hasher
        }.start()
    }
}
impl NetRecSegIn{
    fn pull_from_net(&mut self){

    }
    fn apply_to_store(&mut self){

    }
    fn get_duration_before_frame(&self) -> Duration{
        let milis;
        if FRAME_DURATION_MILLIS >= 5.0 {
            milis = FRAME_DURATION_MILLIS - 2.0; // Should manage calculations in 2ms.
        }else{
            milis = FRAME_DURATION_MILLIS / 5.0 * 4.0; // Probably not going to manage calculations - don't care about timing, just don't want to crash.
        }
        Duration::from_secs_f32(milis / 1000.0)
    }

    pub fn start(mut self) -> NetRecSegEx{
        thread::spawn(move || {
            let frame_counter = self.known_frame.start_frame_stream_from_now();
            loop{

                // TODO3: Can implement a thread thinggy wrapper around the storage to ensure things aren't waiting for each other. (Implement msg gathering)
//                frame_counter.recv().unwrap(); // Just for timing - don't care of frame index.
//                thread::sleep(self.get_duration_before_frame()); // Wait until just before next frame.

                match self.net_inc.recv().unwrap(){
                    ExternalMsg::GameUpdate(update) => {
                        if crate::DEBUG_MSGS_MAIN {
                            println!("Net rec message: {:?}", update);
                        }
                        self.storage.write_owned_data(update);
                    },
                    ExternalMsg::LocalCommand(_) => {panic!("Not implemented!")},
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
        });

        NetRecSegEx{

        }
    }
}

