use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, Mutex, RwLock, RwLockWriteGuard};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{SystemTime, Duration};

use crate::common::network::external_msg::*;
use crate::common::types::*;
use bimap::BiHashMap;
use std::borrow::Borrow;
use crate::server::net_hub_front_seg::DistributableNetMessage;



// For down to the wire stuff about TCP and UDP.
// Outputs friendly byte arrays with addresses.
// Inputs messages with 'reliable' boolean.
// Not a proper segment. Integrated into net hub front

pub enum NetHubBackMsgOut{
    NewPlayer(SocketAddr),
    PlayerDiscon(SocketAddr),
    NewMsg(SocketAddr)
}
pub enum NetHubBackMsgIn{
    SendMsg(SocketAddr, ExternalMsg, /*Reliable*/bool),
    DropPlayer(SocketAddr)
}

pub struct NetHubBackEx {
    pub msg_in: Sender<NetHubBackMsgIn>,
    pub msg_out: Receiver<NetHubBackMsgOut>,
}

pub struct NetHubBackIn {
    host_addr_str: String
}
pub fn net_hub_start_hosting(host_addr_str: String) -> NetHubBackEx{
    NetHubBackIn{
        host_addr_str
    }.start()
}

// Manages the server's incoming and outgoing network messages.
impl NetHubBackIn {
    fn start(&self) -> NetHubBackEx{
        let (in_sink, in_rec) = channel();
        let (out_sink, out_rec) = channel();

        let mut udp_socket = self.bind_udp_addr();

        let mut socket_outgoing = udp_socket.try_clone().expect("Can't clone socket.");
        let mut outgoing_map = addresses_to_ids.clone();


        // Outoing messages.
        thread::spawn(move ||{
            loop{
                let msg_to_yeet = yeet_rec.recv().unwrap();
                let read_only_map = outgoing_map.read().unwrap();
                match msg_to_yeet {
                    DistributableNetMessage::ToSingle(target, msg) => {
                        msg.encode_and_send_udp(&mut socket_outgoing, *read_only_map.get_by_right(&target).unwrap());
                    }
                    DistributableNetMessage::ToAll(msg) => {
                        for (address, player_id) in read_only_map.iter(){
                            msg.encode_and_send_udp(&mut socket_outgoing, *read_only_map.get_by_right(player_id).unwrap());
                        }
                    }
                    DistributableNetMessage::ToAllExcept(do_not_send_to_id, msg) => {
                        for (address, player_id) in read_only_map.iter(){
                            if *player_id != do_not_send_to_id{
                                msg.encode_and_send_udp(&mut socket_outgoing, *read_only_map.get_by_right(player_id).unwrap());
                            }
                        }
                    }
                }
            }
        });

        // Incoming messages.
        thread::spawn(move ||{
            let new_msgs_rec = start_inwards_codec_thread_udp(udp_socket.try_clone().expect("Can't clone socket"));
            let mut test_socket = udp_socket;
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
                        response.encode_and_send_udp(&mut test_socket, address);
                    }
                    ExternalMsg::ConnectionInitQuery(query_data) => {
                        let mut write_handle = addresses_to_ids.write().unwrap();
                        let new_player_id = self.gen_next_player_id(&write_handle);
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
        NetHubBackEx{
            msg_in: in_sink,
            msg_out: out_rec,
        }
    }
    fn bind_udp_addr(&self) -> UdpSocket{
        let host_addr = self.host_addr_str.parse::<SocketAddr>().unwrap();
        println!("Starting hosting on : {}", host_addr);
        UdpSocket::bind(&host_addr).expect("Unable to bind hosting address.")
    }
}