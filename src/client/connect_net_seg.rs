use std::net::{SocketAddr, TcpStream};
use std::ops::Add;
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, SystemTime};
use std::ops::Div;
use std::ops::Sub;

use crate::common::network::external_msg::*;
use crate::common::types::*;

pub struct ConnectNetIn {
    conn_address_str: String,
}
pub struct ConnectNetEx {
    pub net_sink: Sender<ExternalMsg>,
    pub net_rec: Option<Receiver<ExternalMsg>>,
}
struct FullPingSample{
    c_send_time: SystemTime,
    s_receive_time: SystemTime,
    c_receive_time: SystemTime
}
pub const TIME_SAMPLES_REQUIRED : usize = 10;
impl ConnectNetEx {
    fn perform_ping_tests_get_clock_offset(&self) -> i64{
        let mut ping_request_stopper = self.start_ping_sender_thread();
        let data = self.gather_ping_data();

        ping_request_stopper.send(()).unwrap();


        let clock_offset_ns = self.process_ping_data(data);
        println!("Clock offset: {}nanos or {}ms", clock_offset_ns, clock_offset_ns / 1_000_000);
        return clock_offset_ns;
    }
    fn start_ping_sender_thread(&self) -> Sender<ThreadCloser>{
        let my_sender = self.net_sink.clone();
        let (stop_sink, stop_rec) = channel();
        thread::spawn(move ||{
            loop{
                my_sender.send(ExternalMsg::PingTestQuery(SystemTime::now())).unwrap();
                thread::sleep(Duration::from_millis(100)); // Modival
                if stop_rec.try_recv().is_ok(){
                    return;
                }
            }

        });
        return stop_sink;
    }
    fn process_ping_data(&self, ping_data: Vec<FullPingSample>) -> i64{
        let mut total_ping = Duration::from_millis(0);
        for data in &ping_data{
            total_ping = total_ping.add(data.c_receive_time.duration_since(data.c_send_time).unwrap());
        }
        let average_one_way_ping = total_ping.div((2 /*One way*/ * ping_data.len()) as u32); // TODO3: Use a better way to eliminate bad values.

        let data_clock_differences = ping_data.iter().map(|data|{
            let recieve_ms = data.s_receive_time.sub(average_one_way_ping).duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
            let send_ms = data.c_send_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
            // Time from send to recieve.
            return recieve_ms as i64 - send_ms as i64;
        });
        let mut total_difference_in_clocks = 0;
        data_clock_differences.for_each(|difference|{
            total_difference_in_clocks += difference;
        });
        let average_difference_in_clocks = total_difference_in_clocks / ping_data.len() as i64;
        return average_difference_in_clocks;
//        return SystemTime::now().add(Duration::from_nanos(average_difference_in_clocks as u64));
    }
    fn gather_ping_data(&self) -> Vec<FullPingSample>{
        let mut results = vec![];
        while results.len() < TIME_SAMPLES_REQUIRED{
            let returned_maybe = self.net_rec.as_ref().unwrap().recv();
            let c_receive_time = SystemTime::now();
            if returned_maybe.is_ok(){
                match returned_maybe.unwrap(){
                    ExternalMsg::PingTestResponse(response) => {
                        let full_sample = FullPingSample{
                            c_send_time: response.client_time,
                            s_receive_time: response.server_time,
                            c_receive_time,
                        };
                        results.push(full_sample);
                    }
                    _ => {
                        println!("Received message which wasn't a a ping response.");
                    }
                }

            }
        }
        return results;
    }
    fn send_greeting(&self, player_name: &String){
        let connection_init_query = ExternalMsg::ConnectionInitQuery(
            NetMsgGreetingQuery {
                my_player_name: player_name.clone()
            }
        );
        self.net_sink.send(connection_init_query).unwrap();
    }
    pub fn receive_unsynced_greeting(&self) -> NetMsgGreetingResponse {
        loop{
            let welcome_message = self.net_rec.as_ref().unwrap().recv().unwrap();
            match welcome_message{
                ExternalMsg::ConnectionInitResponse(info) =>{
                    if crate::DEBUG_MSGS_MAIN {
                        println!("Received connection init response, init frame: {:?}", info);
                    }
                    return info;
                }
                _ => {
                    if crate::WARN_MSGS {
                        println!("Ignoring first messages until welcome info: {:?}", welcome_message);
                    }
                }

            }

        }
    }
    pub fn receive_synced_greeting(&self, player_name: &String) -> NetMsgGreetingResponse {
        let clock_offset_ns = self.perform_ping_tests_get_clock_offset();
        self.send_greeting(player_name);
        let mut unsynced_greeting = self.receive_unsynced_greeting();
        {
            let synced_frame_info = &mut unsynced_greeting.known_frame;
            println!("Before: {:?}", synced_frame_info);
            synced_frame_info.apply_offset(-clock_offset_ns); // Things work out that this is negative.
            // Known frame checks time between known and now.
            // If the server clock is fast, then we want to decrease our known one so we're using info from the future and vice versa.
            // Simpler explaination:
            // If server is fast, then we need to pull it back to convert it into local client time.
            println!("After: {:?}", synced_frame_info);
        }
        return unsynced_greeting;
    }
}

impl ConnectNetIn {
    pub fn new(conn_address_str :String) -> ConnectNetIn{
        ConnectNetIn{
            conn_address_str
        }
    }
    pub fn start_net(self) -> ConnectNetEx {
        let conn_address = SocketAddr::from_str(&self.conn_address_str).expect("Ill formed ip");
        let connection_result = TcpStream::connect(conn_address);
        let mut stream = connection_result.expect("Failed to connect.");

        let (out_sink, out_rec) = channel();
        let mut stream_outgoing = stream.try_clone().unwrap();
        thread::spawn(move ||{
            loop{
                let message_to_send: ExternalMsg = out_rec.recv().unwrap();
                message_to_send.encode_and_send(&mut stream_outgoing);
            }
        });
        let in_rec = start_inwards_codec_thread(stream);
        return ConnectNetEx {
            net_sink: out_sink,
            net_rec: Some(in_rec),
        };
    }
}












