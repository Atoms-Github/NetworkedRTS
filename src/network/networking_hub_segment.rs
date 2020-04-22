use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::network::networking_message_types::{NetMessageType, start_inwards_codec_thread};
use crate::network::networking_structs::PlayerID;


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
    ToAll(NetMessageType)
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
                        println!("Sending item to one: {:?}", msg);
                        msg.encode_and_send(locked.get_mut(&target).unwrap());
                    }
                    DistributableNetMessage::ToAll(msg) => {
                        println!("Sending an item to all: {:?}", msg);
                        for (player_id, stream) in locked.iter(){
                            msg.encode_and_send(stream);
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
    fn start_socket_read(&mut self, stream: TcpStream, my_id: PlayerID){
        let my_output_sender = self.pickup_sink.clone();
        thread::spawn(move ||{
            let receiver = start_inwards_codec_thread(stream);
            loop{
                my_output_sender.send(OwnedNetworkMessage{
                    owner: my_id,
                    message: receiver.recv().unwrap()
                }).unwrap();
            }

        });
    }
    fn add_socket_to_write_map(&mut self, stream: TcpStream, player_id: PlayerID){
        let mut handle = self.connections_map.lock().unwrap();
        handle.insert(player_id, stream);
    }
    fn handle_new_socket(&mut self, stream: TcpStream, player_id: PlayerID){
        self.start_socket_read(stream.try_clone().unwrap(), player_id);
        self.add_socket_to_write_map(stream, player_id);
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