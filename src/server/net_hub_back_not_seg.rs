use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket, Shutdown};
use std::sync::{Arc, Mutex, RwLock, RwLockWriteGuard};
use std::thread;
use std::time::{SystemTime, Duration};

use crate::common::network::external_msg::*;
use crate::common::types::*;
use bimap::BiHashMap;
use std::borrow::Borrow;
use crossbeam_channel::{Sender, Receiver, Select, unbounded};
use crossbeam_channel::internal::select;


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
    pub fn new(host_addr_str: String) -> Self{
        Self{
            host_addr_str
        }
    }
    pub fn start(self) -> NetHubBackEx{
        let (above_in_sink, above_in_rec) = unbounded();
        let (above_out_sink, above_out_rec) = unbounded();

        let (udp_socket, tcp_listener) = self.bind_sockets();

        // Send udp straight up.
        self.handle_receiving_udp(udp_socket.try_clone().expect("Can't clone socket."), above_out_sink.clone());

        let new_tcps_rec = self.handle_new_tcp_connections(tcp_listener);
        let (inc_tcp_msgs_sink, inc_tcp_msgs_rec) : (Sender<(ExternalMsg, SocketAddr)>,Receiver<(ExternalMsg, SocketAddr)>) = unbounded();

        let mut sel = Select::new();


        thread::spawn(move ||{
//             The things we want to blocking wait for:
//             New tcp streams. (new_tcps_rec)
//             New tcp messages from existing streams. (inc_tcp_msgs_rec)
//             New messages from level above. (above_in_rec)
            let mut connections_map = HashMap::new();

            loop{
                crossbeam_channel::select!{
                    // New tcp streams.
                    recv(new_tcps_rec) -> new_tcp => {
                        let new_stream : TcpStream = new_tcp.unwrap();
                        // Listen for new msgs.
                        new_stream.try_clone().unwrap().start_listening(inc_tcp_msgs_sink.clone());
                        let address = new_stream.peer_addr().unwrap();
                        println!("New client connected {}", address);
                        connections_map.insert(address.clone(), new_stream);
                        above_out_sink.send(NetHubBackMsgOut::NewPlayer(address)).unwrap();
                    },
                    // New tcp msgs.
                    recv(inc_tcp_msgs_rec) -> new_msg_tuple => {
                        let (new_tcp_msg, address) = new_msg_tuple.unwrap();
                        above_out_sink.send(NetHubBackMsgOut::NewMsg(new_tcp_msg, address)).unwrap();
                    },
                    // New msgs from above.
                    recv(above_in_rec) -> msg_from_above => {
                        match msg_from_above.unwrap(){
                            NetHubBackMsgIn::SendMsg(address, external_msg, is_reliable) => {
                                if is_reliable{
                                    connections_map.get_mut(&address).unwrap().send_msg(&external_msg);
                                }else{ // Unreliable:
                                    udp_socket.send_msg(&external_msg, &address);
                                }
                            }
                            NetHubBackMsgIn::DropPlayer(address) => {
                                connections_map.get(&address).unwrap().shutdown(Shutdown::Both).unwrap();
                                connections_map.remove(&address);
                                println!("Dropped client {}", address);
                            }
                        }

                    }
                }
            }
        });
        NetHubBackEx{
            msg_in: above_in_sink,
            msg_out: above_out_rec,
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
            let (new_msgs_sink, new_msgs_rec) = unbounded();
            socket.try_clone().unwrap().start_listening(new_msgs_sink);
            loop{
                let (msg, address) = new_msgs_rec.recv().unwrap();
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
                    socket.send_msg(&response, &address);
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

        println!("Starting hosting on tcp {} and udp on port +1", tcp_address);
        return (udp_socket, tcp_listener);
    }
}