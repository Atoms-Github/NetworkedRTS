use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{SystemTime};

use crate::common::network::external_msg::*;
use crate::common::types::*;

pub struct NetworkingHubEx {
    pub yeet_sink: Sender<DistributableNetMessage>,
    pub pickup_rec: Option<Receiver<OwnedNetworkMessage>>,
}

pub struct NetworkingHubIn {
    host_addr_str: String,
    next_player_id: PlayerID
}

pub struct OwnedNetworkMessage{
    pub owner: PlayerID,
    pub message: ExternalMsg
}

pub enum DistributableNetMessage{
    ToSingle(PlayerID, ExternalMsg),
    ToAllExcept(PlayerID, ExternalMsg),
    ToAll(ExternalMsg)
}

// Manages the server's incoming and outgoing network messages.
impl NetworkingHubIn {
    pub fn new(host_addr_str: String) -> Self {
        NetworkingHubIn{
            host_addr_str,
            next_player_id: 0
        }
    }

    fn gen_next_player_id(&mut self) -> PlayerID{
        let id = self.next_player_id;
        self.next_player_id += 1;
        id
    }

    fn bind_addr(&self) -> UdpSocket{
        let host_addr = self.host_addr_str.parse::<SocketAddr>().unwrap();
        println!("Starting hosting on : {}", host_addr);
        UdpSocket::bind(&host_addr).expect("Unable to bind hosting address.")
    }

    pub fn start_hosting(mut self) -> NetworkingHubEx{
        let (yeet_sink, yeet_rec) = channel();
        let (pickup_sink, pickup_rec) = channel();

        // dcwct Split into send and rec.
        let mut socket = self.bind_addr();
        let mut addresses_to_ids = Arc::new(RwLock::new(bimap::BiHashMap::new()));


        let mut outgoing_socket = socket.try_clone().expect("Can't clone socket.");
        let mut outgoing_map = addresses_to_ids.clone();
        thread::spawn(move ||{
            loop{
                let msg_to_yeet = yeet_rec.recv().unwrap();
                let read_only_map = outgoing_map.read().unwrap();
                match msg_to_yeet {
                    DistributableNetMessage::ToSingle(target, msg) => {
                        msg.encode_and_send_udp(&mut outgoing_socket, *read_only_map.get_by_right(&target).unwrap());
                    }
                    DistributableNetMessage::ToAll(msg) => {
                        for (address, player_id) in read_only_map.iter(){
                            msg.encode_and_send_udp(&mut outgoing_socket, *read_only_map.get_by_right(player_id).unwrap());
                        }
                    }
                    DistributableNetMessage::ToAllExcept(do_not_send_to_id, msg) => {
                        for (address, player_id) in read_only_map.iter(){
                            if *player_id != do_not_send_to_id{
                                msg.encode_and_send_udp(&mut outgoing_socket, *read_only_map.get_by_right(player_id).unwrap());
                            }
                        }
                    }
                }
            }
        });

        thread::spawn(move ||{
            let new_msgs_rec = start_inwards_codec_thread_udp(socket.try_clone().expect("Can't clone socket"));
            loop{
                let (reced_message, address) = new_msgs_rec.recv().unwrap();
                match reced_message{
                    ExternalMsg::PingTestQuery(client_time) => {
                        // This section is like a fine wine in that it is a balance of trying to have equal calculation before and after the time measurement.

//                        thread::sleep(Duration::from_nanos(1)); // Modival This makes server offset go bigger.
                        let server_time = SystemTime::now();
                        // thread::sleep(Duration::from_nanos(1));
                        let mut useless_total = 0;
                        for useless_num in 0..2377{ // Modival This makes server offset go smaller.
                            useless_total += useless_num;
                        }

                        let response = ExternalMsg::PingTestResponse(
                            NetMsgPingTestResponse{
                                client_time,
                                server_time
                            }
                        );
                        response.encode_and_send_udp(&mut socket, address);
                    }
                    ExternalMsg::ConnectionInitQuery(query_data) => {
                        let new_player_id = self.gen_next_player_id();
                        let mut write_handle = addresses_to_ids.write().unwrap();
                        write_handle.insert(address, new_player_id);
                        let owned_msg = OwnedNetworkMessage{
                            owner: new_player_id,
                            message: ExternalMsg::ConnectionInitQuery(query_data),
                        };
                        pickup_sink.send(owned_msg).unwrap(); // Redirect message to server main listener.
                    }
                    non_query_msg => {
                        let read_handle =  addresses_to_ids.read().unwrap();
                        let player_id = read_handle.get_by_left(&address).expect("Received a non-init non-ping message from address without playerID.");
                        let owned_msg = OwnedNetworkMessage{
                            owner: *player_id,
                            message: non_query_msg,
                        };
                        pickup_sink.send(owned_msg).unwrap(); // Redirect message to server main listener.
                    }
                }
            }
        });
        NetworkingHubEx{
            yeet_sink,
            pickup_rec: Some(pickup_rec)
        }
    }
}