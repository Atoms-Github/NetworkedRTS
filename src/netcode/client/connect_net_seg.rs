use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::ops::Add;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, SystemTime};
use std::ops::Div;
use std::ops::Sub;
use crossbeam_channel::*;

use crate::netcode::common::external_msg::*;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use crate::netcode::common::timekeeping::KnownFrameInfo;
use bibble_tokio::NetClientTop;

pub struct ClientNet {
    pub client: NetClientTop<ExternalMsg>,
}
struct FullPingSample{
    c_send_time: SystemTime,
    s_receive_time: SystemTime,
    c_receive_time: SystemTime
}
pub const TIME_SAMPLES_REQUIRED : usize = 2;
impl ClientNet {
    pub fn start(conn_address: String) -> Self{
        Self{
            client: bibble_tokio::start_client::<ExternalMsg>(conn_address)
        }
    }
    fn start_ping_sender_thread(&self) -> Sender<ThreadCloser>{
        let my_sender = self.client.down.clone();
        let (stop_sink, stop_rec) = unbounded();
        thread::spawn(move ||{
            loop{
                my_sender.send((ExternalMsg::PingTestQuery(SystemTime::now()),false)).unwrap();
                thread::sleep(Duration::from_millis(100));
                if stop_rec.try_recv().is_ok(){
                    return;
                }
            }

        });
        stop_sink
    }
    fn calculate_local_time(&self, ping_data: Vec<FullPingSample>, mut server_time: KnownFrameInfo) -> KnownFrameInfo{
        let mut total_ping = Duration::from_millis(0);
        for data in &ping_data{
            total_ping = total_ping.add(data.c_receive_time.duration_since(data.c_send_time).unwrap());
        }
        let average_one_way_ping = total_ping.div((2 /*One way*/ * ping_data.len()) as u32); // TODO3: Use a better way to eliminate bad values.

        let data_clock_differences = ping_data.iter().map(|data|{
            let recieve_ms = data.s_receive_time.sub(average_one_way_ping).duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
            let send_ms = data.c_send_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
            // Time from send to recieve.
            recieve_ms as i64 - send_ms as i64
        });
        let mut total_difference_in_clocks = 0;
        data_clock_differences.for_each(|difference|{
            total_difference_in_clocks += difference;
        });

        let time_offset = total_difference_in_clocks / ping_data.len() as i64;
        log::info!("Clock difference: {}ms", time_offset / 1_000_000);
        server_time.apply_offset(-time_offset);
        return server_time;
            // Things work out that this is negative.
            // Known frame checks time between known and now.
            // If the server clock is fast, then we want to decrease our known one so we're using info from the future and vice versa.
            // Simpler explaination:
            // If server is fast, then we need to pull it back to convert it into local client time.
    }
    // This is the data gathering step.
    fn gather_ping_and_init_data(&mut self) -> (Vec<FullPingSample>, NetMsgGreetingResponse){
        let mut ping_request_stopper = self.start_ping_sender_thread(); // Asks for ping samples.

        self.client.send_msg(ExternalMsg::ConnectionInitQuery, true);

        let mut opt_greetings = None;
        let mut ping_results = vec![];
        while ping_results.len() < TIME_SAMPLES_REQUIRED || opt_greetings.is_none(){
            let inc_msg = self.client.recv();
            let c_receive_time = SystemTime::now();

            match inc_msg {
                ExternalMsg::PingTestResponse(response) => {
                    let full_sample = FullPingSample{
                        c_send_time: response.client_time,
                        s_receive_time: response.server_time,
                        c_receive_time,
                    };
                    ping_results.push(full_sample);
                }
                ExternalMsg::ConnectionInitResponse(info) =>{
                    if crate::DEBUG_MSGS_MAIN {
                        log::info!("Received connection init response: {:?}", info);
                    }
                    opt_greetings = Some(info);
                }
                ExternalMsg::GameUpdate(game_data) => {
                    // Nothing.
                }
                other_msg => {
                    log::debug!("Received message which wasn't a a ping response: {:?}", other_msg);
                }
            }
        }
        // Close ping sender thread.
        ping_request_stopper.send(()).unwrap();
        (ping_results, opt_greetings.unwrap())
    }
    pub fn get_synced_greeting(&mut self) -> NetMsgGreetingResponse {
        let (ping_data, mut greeting) = self.gather_ping_and_init_data();

        greeting.known_frame = self.calculate_local_time(ping_data, greeting.known_frame);
        log::info!("I'm player {}", greeting.assigned_player_id);

        greeting
    }
}
