use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::ops::Add;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, SystemTime};
use std::ops::Div;
use std::ops::Sub;
use crossbeam_channel::*;

use crate::common::network::external_msg::*;
use crate::common::types::*;

pub struct ConnectNetIn {
    conn_address_str: String,

}
pub struct ConnectNetEx {
    pub net_sink: Sender<(ExternalMsg, bool)>,
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
        clock_offset_ns
    }
    fn start_ping_sender_thread(&self) -> Sender<ThreadCloser>{
        let my_sender = self.net_sink.clone();
        let (stop_sink, stop_rec) = unbounded();
        thread::spawn(move ||{
            loop{
                my_sender.send((ExternalMsg::PingTestQuery(SystemTime::now()),false)).unwrap();
                thread::sleep(Duration::from_millis(100)); // Modival
                if stop_rec.try_recv().is_ok(){
                    return;
                }
            }

        });
        stop_sink
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
            recieve_ms as i64 - send_ms as i64
        });
        let mut total_difference_in_clocks = 0;
        data_clock_differences.for_each(|difference|{
            total_difference_in_clocks += difference;
        });
        // TODO2: Rename method or something to make it clear we're returning the average difference in clocks.
        total_difference_in_clocks / ping_data.len() as i64
//        return SystemTime::now().add(Duration::from_nanos(average_difference_in_clocks as u64));
    }
    fn gather_ping_data(&self) -> Vec<FullPingSample>{
        let mut results = vec![];
        while results.len() < TIME_SAMPLES_REQUIRED{
            let returned_maybe = self.net_rec.as_ref().unwrap().recv().unwrap();
            let c_receive_time = SystemTime::now();
            match returned_maybe{
                ExternalMsg::PingTestResponse(response) => {
                    let full_sample = FullPingSample{
                        c_send_time: response.client_time,
                        s_receive_time: response.server_time,
                        c_receive_time,
                    };
                    results.push(full_sample);
                }
                other_msg => {
                    println!("Received message which wasn't a a ping response: {:?}", other_msg);
                }
            }
        }
        results
    }
    fn send_greeting(&self, player_name: &str, preferred_id: i32){
        let connection_init_query = ExternalMsg::ConnectionInitQuery(
            NetMsgGreetingQuery {
                my_player_name: player_name.to_string(),
                preferred_id: 5
            }
        );
        self.net_sink.send((connection_init_query, true)).unwrap();
    }
    pub fn receive_unsynced_greeting(&self) -> NetMsgGreetingResponse {
        loop{
            let welcome_message = self.net_rec.as_ref().unwrap().recv().unwrap();
            match welcome_message{
                ExternalMsg::ConnectionInitResponse(info) =>{
                    thread::sleep(Duration::from_millis(500)); // Shouldn't matter how long to take to download.
                    if crate::DEBUG_MSGS_MAIN {
                        println!("Received connection init response: {:?}", info);
                    }
                    return info;
                }
                _ => {
                    println!("Ignoring first messages until welcome info: {:?}", welcome_message);
                }

            }

        }
    }
    pub fn receive_synced_greeting(&self, player_name: &str, preferred_id: i32) -> NetMsgGreetingResponse {
        let clock_offset_ns = self.perform_ping_tests_get_clock_offset();
        self.send_greeting(player_name, preferred_id);
        let mut unsynced_greeting = self.receive_unsynced_greeting();
        println!("I'm player {}", unsynced_greeting.assigned_player_id);
        {
            let synced_frame_info = &mut unsynced_greeting.known_frame;
            synced_frame_info.apply_offset(-clock_offset_ns); // Things work out that this is negative.
            // Known frame checks time between known and now.
            // If the server clock is fast, then we want to decrease our known one so we're using info from the future and vice versa.
            // Simpler explaination:
            // If server is fast, then we need to pull it back to convert it into local client time.
        }
        unsynced_greeting
    }
}

impl ConnectNetIn {
    pub fn new(conn_address_str :String) -> ConnectNetIn{
        ConnectNetIn{
            conn_address_str
        }
    }
    fn bind_sockets(&self) -> (UdpSocket, TcpStream){
        let tcp_address = SocketAddr::from_str(&self.conn_address_str).expect("Ill formed ip");

        let mut udp_address = tcp_address.clone();
        udp_address.set_port(tcp_address.port() + 1);

        let tcp_stream = TcpStream::connect(tcp_address).expect("Client failed to connect TCP.");
        let udp_socket = UdpSocket::bind("127.0.0.1:0"/*Auto assign*/).expect("Client couldn't bind to socket.");

        udp_socket.connect(udp_address).expect("Client failed to connect UDP.");

        println!("Connected to server on on tcp {} and udp on port +1", tcp_address);
        return (udp_socket, tcp_stream);
    }
    pub fn start_net(self) -> ConnectNetEx {
        let (down_sink, down_rec) = unbounded();
        let (up_sink, up_rec) = unbounded();

        let (udp_socket, mut tcp_stream) = self.bind_sockets();

        udp_socket.try_clone().unwrap().start_listening(up_sink.clone());
        tcp_stream.try_clone().unwrap().start_listening(up_sink);

        thread::spawn(move ||{
            loop{
                let (msg, reliable) = down_rec.recv().unwrap();
                if reliable{
                    tcp_stream.send_msg(&msg);
                }else{
                    udp_socket.send_msg_to_connected(&msg);
                }
            }
        });

        ConnectNetEx {
            net_sink: down_sink,
            net_rec: Some(up_rec.filter_address(None))
        }
    }
}

