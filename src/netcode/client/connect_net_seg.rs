use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::ops::Add;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, SystemTime};
use std::ops::Div;
use std::ops::Sub;
use crossbeam_channel::*;

use crate::netcode::common::network::external_msg::*;
use crate::netcode::common::types::*;
use crate::netcode::common::time::timekeeping::KnownFrameInfo;
use crate::netcode::common::network::channel_threads::*;

pub struct ConnectNetIn {
    conn_address_str: String,

}
pub struct ConnectNetEx {
    pub net_sink: Sender<(ExternalMsg, bool)>,
    pub net_rec: Option<Receiver<ExternalMsg>>,
    pub udp_port: u16,
}
struct FullPingSample{
    c_send_time: SystemTime,
    s_receive_time: SystemTime,
    c_receive_time: SystemTime
}
pub const TIME_SAMPLES_REQUIRED : usize = 2;
impl ConnectNetEx {
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

        server_time.apply_offset(-time_offset);
        return server_time;
            // Things work out that this is negative.
            // Known frame checks time between known and now.
            // If the server clock is fast, then we want to decrease our known one so we're using info from the future and vice versa.
            // Simpler explaination:
            // If server is fast, then we need to pull it back to convert it into local client time.
    }
    // This is the data gathering step.
    fn gather_ping_and_init_data(&self, my_details: NetMsgGreetingQuery) -> (Vec<FullPingSample>, NetMsgGreetingResponse){
        let mut ping_request_stopper = self.start_ping_sender_thread(); // Asks for ping samples.
        self.send_greeting(my_details); // Asks for greetings.

        let mut opt_greetings = None;
        let mut ping_results = vec![];
        while ping_results.len() < TIME_SAMPLES_REQUIRED || opt_greetings.is_none(){
            let inc_msg = self.net_rec.as_ref().unwrap().recv().unwrap();
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
        ping_request_stopper.send(()).unwrap();
        (ping_results, opt_greetings.unwrap())
    }
    fn send_greeting(&self, my_details: NetMsgGreetingQuery){
        let connection_init_query = ExternalMsg::ConnectionInitQuery(
            my_details
        );
        self.net_sink.send((connection_init_query, true)).unwrap();
    }
    pub fn get_synced_greeting(&self, my_details: NetMsgGreetingQuery) -> NetMsgGreetingResponse {
        let (ping_data, mut greeting) = self.gather_ping_and_init_data(my_details);

        greeting.known_frame = self.calculate_local_time(ping_data, greeting.known_frame);
        log::info!("I'm player {}", greeting.assigned_player_id);

        greeting
    }
    pub fn start(conn_address_str :String) -> Self{
        ConnectNetIn{
            conn_address_str
        }.start_net()
    }
}

impl ConnectNetIn {
    fn bind_sockets(&self) -> (UdpSocket, TcpStream){
        let target_tcp_address = SocketAddr::from_str(&self.conn_address_str).expect("Ill formed ip");

        let mut target_udp_address = target_tcp_address.clone();
        target_udp_address.set_port(target_tcp_address.port() + 1);

        loop{
            let tcp_stream;
            match TcpStream::connect(target_tcp_address){
                Err(error) => {
                    log::warn!("Failed to connect to server. Retrying ... ({})", error.to_string());
                    thread::sleep(Duration::from_millis(1000));
                    continue;
                }
                Ok(stream) => {
                    tcp_stream = stream;
                }
            }
            let udp_socket = UdpSocket::bind(tcp_stream.local_addr().unwrap()).expect("Client couldn't bind to socket.");

            udp_socket.connect(target_udp_address).expect("Client failed to connect UDP.");
            log::info!("Client using udp {:?}" , udp_socket.local_addr());
            log::info!("Connected to server on on tcp {} and udp on port +1", target_tcp_address);
            log::info!("");
            return (udp_socket, tcp_stream);
        }

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
            net_rec: Some(up_rec.filter_address(None)),
            udp_port: 0,//udp_socket.local_addr().unwrap().port(),
        }
    }
}



//#[cfg(test)]
pub mod connect_tests {
    use std::net::SocketAddr;
    use crate::netcode::client::connect_net_seg::*;
    use crate::netcode::common::network::external_msg::NetMsgGreetingQuery;

    fn init_connection() -> ConnectNetEx{
        let mut seg_connect_net = ConnectNetEx::start("127.0.0.1:1414".to_string());
        return seg_connect_net;
    }
    //#[test]
    pub fn crash_on_connect() {
        let connect = init_connection();
        thread::sleep(Duration::from_millis(300));
        panic!();
    }
    //#[test]
    pub fn wait_on_connect() {
        let connect = init_connection();

        loop{
            thread::sleep(Duration::from_millis(10000));
        }
    }
}




