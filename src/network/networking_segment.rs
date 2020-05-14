use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::str::FromStr;

use crate::network::networking_message_types::*;
use std::time::{SystemTime, Duration};
use std::ops::*;
use crate::network::networking_structs::*;
use crate::game::logic::logic_segment::*;
use crate::game::synced_data_stream::*;

use crate::players::inputs::*;

pub struct NetworkingSegmentIn {
    conn_address_str: String
}
pub struct NetworkingSegmentEx {
    pub net_sink: Sender<NetMessageType>,
    pub net_rec: Receiver<NetMessageType>,
}
struct FullPingSample{
    c_send_time: SystemTime,
    s_receive_time: SystemTime,
    c_receive_time: SystemTime
}
pub const TIME_SAMPLES_REQUIRED : usize = 10;
impl NetworkingSegmentEx {
    pub fn perform_ping_tests_get_clock_offset(&mut self) -> i64{
        self.start_ping_sender_thread();
        let data = self.gather_ping_data();
        return self.process_ping_data(data);
    }
    fn start_ping_sender_thread(&mut self){
        let my_sender = self.net_sink.clone();
        thread::spawn(move ||{
            loop{
                my_sender.send(NetMessageType::PingTestQuery(SystemTime::now())).unwrap();
                thread::sleep(Duration::from_millis(100)); // Modival
            }

        }); // TODO2: Add finish method to here.
    }
    fn process_ping_data(&mut self, ping_data: Vec<FullPingSample>) -> i64{
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
    fn gather_ping_data(&mut self) -> Vec<FullPingSample>{
        let mut results = vec![];
        while results.len() < TIME_SAMPLES_REQUIRED{
            let returned_maybe = self.net_rec.recv();
            let c_receive_time = SystemTime::now();
            if returned_maybe.is_ok(){
                match returned_maybe.unwrap(){
                    NetMessageType::PingTestResponse(response) => {
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
    pub fn send_greeting(&mut self, player_name: &String){
        let connection_init_query = NetMessageType::ConnectionInitQuery(
            NetMsgConnectionInitQuery{
                my_player_name: player_name.clone()
            }
        );
        self.net_sink.send(connection_init_query).unwrap();
    }
    pub fn receive_welcome_message(&mut self) -> NetMsgConnectionInitResponse{
        loop{
            let welcome_message = self.net_rec.recv().unwrap();
            match welcome_message{
                NetMessageType::ConnectionInitResponse(info) =>{
                    if crate::SEND_DEBUG_MSGS{
                        println!("Received connection init response {:?}", info);
                    }

                    return info;
                }
                _ => {
                }

            }
            if crate::SEND_DEBUG_MSGS{
                println!("Ignoring first messages until welcome info: {:?}", welcome_message);
            }
        }
    }
    pub fn send_init_me_msg(&mut self, frame_to_init_on: FrameIndex, my_player_id: PlayerID){
        let mut first_input = InputState::new();
        first_input.new_player = true;
        let syncer_data = SyncerData{
            data: vec![first_input],
            start_frame: frame_to_init_on,
            owning_player: my_player_id
        };
        self.net_sink.send(NetMessageType::GameUpdate(LogicInwardsMessage::SyncerInputsUpdate(syncer_data))).unwrap();
    }
}

impl NetworkingSegmentIn {
    pub fn new(conn_address_str :String) -> NetworkingSegmentIn{
        NetworkingSegmentIn{
            conn_address_str
        }
    }
    pub fn start_net(self) -> NetworkingSegmentEx {
        let conn_address = SocketAddr::from_str(&self.conn_address_str).expect("Ill formed ip");
        let connection_result = TcpStream::connect(conn_address);
        let mut stream = connection_result.expect("Failed to connect.");

        let (out_sink, out_rec) = channel();
        let mut stream_outgoing = stream.try_clone().unwrap();
        thread::spawn(move ||{
            loop{
                let message_to_send: NetMessageType = out_rec.recv().unwrap();
                message_to_send.encode_and_send(&mut stream_outgoing);
            }
        });
        let in_rec = start_inwards_codec_thread(stream);
        return NetworkingSegmentEx {
            net_sink: out_sink,
            net_rec: in_rec
        };
    }
}












