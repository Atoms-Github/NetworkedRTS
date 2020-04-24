use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::network::networking_message_types::{NetMessageType, start_inwards_codec_thread};
use crate::network::networking_structs::PlayerID;
use std::time::{SystemTime};
use crate::network::networking_message_types::*;


pub struct NetworkingHubEx {
    pub yeet_sink: Sender<DistributableNetMessage>,
    pub pickup_rec: Option<Receiver<OwnedNetworkMessage>>,
}

pub struct NetworkingHubMid {
    net_in: NetworkingHubIn,
    connections_map: Arc<Mutex<HashMap<PlayerID, TcpStream>>>,
    pickup_sink: Sender<OwnedNetworkMessage>,
    next_player_id: PlayerID
}

pub struct NetworkingHubIn {
    host_addr_str: String,
}

pub struct OwnedNetworkMessage{
    pub owner: PlayerID,
    pub message: NetMessageType
}

pub enum DistributableNetMessage{
    ToSingle(PlayerID, NetMessageType),
    ToAllExcept(PlayerID, NetMessageType),
    ToAll(NetMessageType)
}
struct InitingPlayer{
    player_id: PlayerID,
    net_sink: TcpStream,
    net_rec: Receiver<NetMessageType>,
    connections_map_handle: Arc<Mutex<HashMap<PlayerID, TcpStream>>>,
    later_messages_sink: Sender<OwnedNetworkMessage>,
}
impl InitingPlayer{
    fn start_socket_read(self){
        thread::spawn(move ||{
            loop{
                self.later_messages_sink.send(OwnedNetworkMessage{
                    owner: self.player_id,
                    message: self.net_rec.recv().unwrap()
                }).unwrap();
            }

        });
    }
    fn add_socket_to_write_map(&mut self, stream: TcpStream){
        let mut handle = self.connections_map_handle.lock().unwrap();
        handle.insert(self.player_id, stream);
    }
    pub fn handle_init(mut self){
        thread::spawn(move ||{
            loop{
                let reced_message = self.net_rec.recv().unwrap();
                match reced_message{
                    NetMessageType::PingTestQuery(client_time) => {
                        // This section is like a fine wine in that it is a balance of trying to have equal calculation before and after the time measurement.


//                        thread::sleep(Duration::from_nanos(1)); // Modival This makes server offset go bigger.
                        let server_time = SystemTime::now();
                        // thread::sleep(Duration::from_nanos(1));
                        let mut useless_total = 0;
                        for useless_num in 0..2377{ // Modival This makes server offset go smaller.
                            useless_total += useless_num;
                        }

                        let response = NetMessageType::PingTestResponse(
                            NetMsgPingTestResponse{
                                client_time,
                                server_time
                            }
                        );


                        response.encode_and_send(&mut self.net_sink);
                    }
                    NetMessageType::ConnectionInitQuery(query_data) => {
                        let owned_msg = OwnedNetworkMessage{
                            owner: self.player_id,
                            message: NetMessageType::ConnectionInitQuery(query_data),
                        };
                        self.later_messages_sink.send(owned_msg).unwrap(); // Redirect message to server main listener.
                        break;
                    }
                    _ => {
                        panic!("Client send incorrect message type before sending initialize.");
                    }
                }
            }
            self.add_socket_to_write_map(self.net_sink.try_clone().unwrap());
            self.start_socket_read();
        });
    }
}
impl NetworkingHubMid{
    pub fn startup(mut self, yeet_rec: Receiver<DistributableNetMessage>){
        self.start_yeeting_msgs(yeet_rec);
        thread::spawn( move ||{ // Listen for new connections
            let socket = self.bind_addr();

            for stream in socket.incoming() {
                let next_id = self.gen_next_player_id();
                self.handle_new_socket(stream.unwrap(), next_id);
            }
        });
    }
    fn start_yeeting_msgs(&mut self, yeet_rec: Receiver<DistributableNetMessage>){
        let my_stream_map = self.connections_map.clone();
        thread::spawn(move ||{
            loop{
                let to_yeet = yeet_rec.recv().unwrap();
                let mut locked = my_stream_map.lock().unwrap();

                match to_yeet {
                    DistributableNetMessage::ToSingle(target, msg) => {
                        msg.encode_and_send(locked.get_mut(&target).unwrap());
                    }
                    DistributableNetMessage::ToAll(msg) => {
                        for (player_id, stream) in locked.iter(){
                            msg.encode_and_send(stream);
                        }
                    }
                    DistributableNetMessage::ToAllExcept(do_not_send_to_id, msg) => {
                        for (player_id, stream) in locked.iter(){
                            if *player_id != do_not_send_to_id{
                                msg.encode_and_send(stream);

                            }
                        }
                    }
                }
            }
        });
    }
    fn gen_next_player_id(&mut self) -> PlayerID{
        let id = self.next_player_id;
        self.next_player_id += 1;
        return id;
    }
    fn bind_addr(&self) -> TcpListener{
        let host_addr = self.net_in.host_addr_str.parse::<SocketAddr>().unwrap();
        println!("Starting hosting on : {}", host_addr);
        return TcpListener::bind(&host_addr).expect("Unable to bind hosting address.");
    }

    fn handle_new_socket(&mut self, stream: TcpStream, player_id: PlayerID){
        let initing_player = InitingPlayer{
            player_id,
            net_sink: stream.try_clone().unwrap(),
            net_rec: start_inwards_codec_thread(stream),
            connections_map_handle: self.connections_map.clone(),
            later_messages_sink: self.pickup_sink.clone()
        };
        initing_player.handle_init();

    }
}

// Manages the server's incoming and outgoing network messages.
impl NetworkingHubIn {
    pub fn new(host_addr_str: String) -> Self {
        return NetworkingHubIn{
            host_addr_str
        }
    }
    pub fn start_hosting(self) -> NetworkingHubEx{
        let (yeet_sink, yeet_rec) = channel();
        let (pickup_sink, pickup_rec) = channel();
        let mid = NetworkingHubMid{
            net_in: self,
            connections_map: Arc::new(Mutex::new(Default::default())),
            pickup_sink,
            next_player_id: 0
        };
        mid.startup(yeet_rec);

        return NetworkingHubEx{
            yeet_sink,
            pickup_rec: Some(pickup_rec)
        }
    }
}