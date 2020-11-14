use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, Mutex, RwLock, RwLockWriteGuard};
use std::thread;
use std::time::{SystemTime, Duration};

use crate::common::network::external_msg::*;
use crate::common::types::*;
use bimap::BiHashMap;
use std::borrow::Borrow;
use crate::server::net_hub_front_seg::DistributableNetMessage;

use crossbeam_channel::*;


// For down to the wire stuff about TCP and UDP.
// Outputs friendly byte arrays with addresses.
// Inputs messages with 'reliable' boolean.
// Not a proper segment. Integrated into net hub front

pub enum NetHubBackMsgOut{
    NewPlayer(SocketAddr),
    PlayerDiscon(SocketAddr),
    NewMsg(ExternalMsg, SocketAddr)
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

// This section is pretty single threaded, so main thread waits for new tcp streams, or new messages from above, or new messages from existing tcp stream.
// New messages from udp streams are streamlined, so just ignore this area and go straight on up to level above.
impl NetHubBackIn {
    fn start(&self) -> NetHubBackEx{
        let (in_sink, in_rec) = unbounded();
        let (out_sink, out_rec) = unbounded();


        let (udp_socket, tcp_listener) = self.bind_sockets();

        self.handle_receiving_udp(udp_socket.try_clone().expect("Can't clone socket."), out_sink.clone());
        let new_tcp_streams = self.handle_new_tcp_connections(tcp_listener);

        thread::spawn(move ||{
            // The things we want to blocking wait for:
            // New tcp streams.
            // New tcp messages from existing streams.
            // New messages from level above.

//            let mut tcp_map = HashMap::new();
            let new_tcp : TcpStream = new_tcp_streams.recv().unwrap();
            let msg_stream = start_inwards_codec_thread_tcp(new_tcp);
//            select!{
//            recv(new_tcp_streams) -> new_tcp_stream => {
//            }
//            recv(in_rec) -> new_in_msg => {
//            }
//            }

        });

        NetHubBackEx{
            msg_in: in_sink,
            msg_out: out_rec,
        }

    }
    fn handle_new_tcp_connections(&self, listener: TcpListener) -> Receiver<TcpStream>{
        let (sink, rec) = unbounded();
        thread::spawn(move ||{
            for new_stream in listener.incoming(){
                sink.send(new_stream.expect("Failed to listen to new player.")).unwrap();
            }
        });
        return rec;
    }
    fn handle_receiving_udp(&self, socket: UdpSocket, out_msgs: Sender<NetHubBackMsgOut>){
        thread::spawn(move||{
            let mut outgoing_socket = socket.try_clone().unwrap();
            let new_msgs = start_inwards_codec_thread_udp(socket);
            loop{
                let (msg, address) = new_msgs.recv().unwrap();
                // If a ping test, just 420 blaze out a response.
                if let ExternalMsg::PingTestQuery(client_time) = msg{
                        // This section is like a fine wine in that it is a balance of trying to have equal calculation before and after the time measurement.
                        let server_time = SystemTime::now();
                        let response = ExternalMsg::PingTestResponse(
                            NetMsgPingTestResponse{
                                client_time,
                                server_time
                            }
                        );
                        response.encode_and_send_udp(&mut outgoing_socket, address);
                }else{
                    out_msgs.send(NetHubBackMsgOut::NewMsg(msg, address)).unwrap();
                }
            }
        });
        // Send straight up past net layer to server (unless ping)
    }
    fn bind_sockets(&self) -> (UdpSocket, TcpListener){
        let tcp_address = self.host_addr_str.parse::<SocketAddr>().unwrap();
        let mut udp_address = tcp_address.clone();
        udp_address.set_port(tcp_address.port() + 1);

        let tcp_listener = TcpListener::bind(&tcp_address).expect("Unable to bind ");
        let udp_socket = UdpSocket::bind(&udp_address).expect("Unable to bind udp hosting address.");

        println!("Starting hosting on : {} and port +1", udp_address);
        return (udp_socket, tcp_listener);
    }
}
// Outoing messages.
//        thread::spawn(move ||{
//            loop{
//                let msg_to_yeet = yeet_rec.recv().unwrap();
//                let read_only_map = outgoing_map.read().unwrap();
//                match msg_to_yeet {
//                    DistributableNetMessage::ToSingle(target, msg) => {
//                        msg.encode_and_send_udp(&mut socket_outgoing, *read_only_map.get_by_right(&target).unwrap());
//                    }
//                    DistributableNetMessage::ToAll(msg) => {
//                        for (address, player_id) in read_only_map.iter(){
//                            msg.encode_and_send_udp(&mut socket_outgoing, *read_only_map.get_by_right(player_id).unwrap());
//                        }
//                    }
//                    DistributableNetMessage::ToAllExcept(do_not_send_to_id, msg) => {
//                        for (address, player_id) in read_only_map.iter(){
//                            if *player_id != do_not_send_to_id{
//                                msg.encode_and_send_udp(&mut socket_outgoing, *read_only_map.get_by_right(player_id).unwrap());
//                            }
//                        }
//                    }
//                }
//            }
//        });
//
//        // Incoming messages.
//        thread::spawn(move ||{
//            let new_msgs_rec = start_inwards_codec_thread_udp(udp_socket.try_clone().expect("Can't clone socket"));
//            let mut test_socket = udp_socket;
//            loop{
//                let (reced_message, address) = new_msgs_rec.recv().unwrap();
//                match reced_message{
//                    ExternalMsg::PingTestQuery(client_time) => {
//                        // This section is like a fine wine in that it is a balance of trying to have equal calculation before and after the time measurement.
//
////                        thread::sleep(Duration::from_nanos(1)); // Modival This makes server offset go bigger.
//                        let server_time = SystemTime::now();
//                        // thread::sleep(Duration::from_nanos(1));
//                        let mut useless_total = 0;
//                        for useless_num in 0..2377{ // Modival This makes server offset go smaller.
//                            useless_total += useless_num;
//                        }
//
//                        let response = ExternalMsg::PingTestResponse(
//                            NetMsgPingTestResponse{
//                                client_time,
//                                server_time
//                            }
//                        );
//                        response.encode_and_send_udp(&mut test_socket, address);
//                    }
//                    ExternalMsg::ConnectionInitQuery(query_data) => {
//                        let mut write_handle = addresses_to_ids.write().unwrap();
//                        let new_player_id = self.gen_next_player_id(&write_handle);
//                        write_handle.insert(address, new_player_id);
//                        let owned_msg = OwnedNetworkMessage{
//                            owner: new_player_id,
//                            message: ExternalMsg::ConnectionInitQuery(query_data),
//                        };
//                        pickup_sink.send(owned_msg).unwrap(); // Redirect message to server main listener.
//                    }
//                    non_query_msg => {
//                        let read_handle =  addresses_to_ids.read().unwrap();
//                        let player_id = read_handle.get_by_left(&address).expect("Received a non-init non-ping message from address without playerID.");
//                        let owned_msg = OwnedNetworkMessage{
//                            owner: *player_id,
//                            message: non_query_msg,
//                        };
//                        pickup_sink.send(owned_msg).unwrap(); // Redirect message to server main listener.
//                    }
//                }
//            }
//        });
//        NetHubBackEx{
//            msg_in: in_sink,
//            msg_out: out_rec,
//        }