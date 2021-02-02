
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
use crossbeam_channel::internal::{select, SelectHandle};
use crate::common::network::channel_threads::*;


// For down to the wire stuff about TCP and UDP.
// Outputs friendly byte arrays with addresses.
// Inputs messages with 'reliable' boolean.
// Not a proper segment. Integrated into net hub front

#[derive(Debug)]
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


// This only cares about addresses and stream. It doesn't do any memory about disconnected players.
// If they're disconnected, they're deleted.
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
        let (inc_tcp_msgs_sink, inc_tcp_msgs_rec) = unbounded();

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
                        log::info!("New client connected {}", address);
                        connections_map.insert(address.clone(), new_stream);
                        above_out_sink.send(NetHubBackMsgOut::NewPlayer(address)).unwrap();
                    },
                    // New tcp msgs.
                    recv(inc_tcp_msgs_rec) -> new_msg => {
                        match new_msg.unwrap(){
                            SocketIncEvent::Msg(msg, address) => {
                                above_out_sink.send(NetHubBackMsgOut::NewMsg(msg, address)).unwrap();
                            }
                            SocketIncEvent::Diconnect(address) => {
                                assert!(connections_map.remove(&address).is_some(), "TCP disconnected player twice. May happen super rare on mad rapid connect spam.");
                                above_out_sink.send(NetHubBackMsgOut::PlayerDiscon(address)).unwrap();
                                log::debug!("Net back disconnecting {}", address);
                            }
                        }
                    },
                    // New msgs from above.
                    recv(above_in_rec) -> msg_from_above => {
                        match msg_from_above.unwrap(){
                            NetHubBackMsgIn::SendMsg(address, external_msg, is_reliable) => {
                                match connections_map.get_mut(&address)  {
                                    Some(tcp_socket) => {
                                        if is_reliable{
                                            tcp_socket.send_msg(&external_msg);

                                        }else{ // Unreliable:
                                            udp_socket.send_msg(&external_msg, &address);
                                        }
                                    }
                                    None => {
                                        // Do nothing. Ignore msg requests that send to disconnected client.
                                        // Pointless_optimum, could prevent msg from even being created if this is handled in layer above.
                                    }
                                }

                            }
                            NetHubBackMsgIn::DropPlayer(address) => {
                                connections_map.get(&address).unwrap().shutdown(Shutdown::Both).unwrap();
                                connections_map.remove(&address);
                                log::info!("Dropped client {}", address);
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
                let inc_event = new_msgs_rec.recv().unwrap();
                match inc_event{
                    SocketIncEvent::Msg(msg, address) => {
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
                    _ => {
                        log::warn!("UDP doesn't handle disconnect!")
                    }
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

        log::info!("Starting hosting on tcp {} and udp on port +1", tcp_address);
        return (udp_socket, tcp_listener);
    }
}





// #[cfg(test)]
pub mod hub_back_test {
    use std::net::SocketAddr;
    use crate::server::net_hub_back_not_seg::*;
    use crate::common::data::hash_seg::FramedHash;

    // #[test]
    pub fn print_listened() {
        println!("Starting test.");
        let net_hub_backend = NetHubBackIn::new("127.0.0.1:1414".to_string()).start();
        // thread::spawn(move ||{
        //     loop{
        //         thread::sleep(Duration::from_millis(100));
        //     }
        // });
        loop{
            let msg = net_hub_backend.msg_out.recv().unwrap();
            
            println!("Test server listened: {:?}", msg);
            match msg{
                NetHubBackMsgOut::NewPlayer(address) => {
                    let th_sink = net_hub_backend.msg_in.clone();
                    thread::spawn(move ||{
                        loop{
                            th_sink.send(NetHubBackMsgIn::SendMsg(address, ExternalMsg::NewHash(FramedHash::new(0, 0)), false)).unwrap();
                            //th_sink.send(NetHubBackMsgIn::SendMsg(address, ExternalMsg::NewHash(FramedHash::new(0, 0)), true)).unwrap();
                            thread::sleep(Duration::from_millis(500));
                        }
                    });
                }
                _ => {

                }
            }
        }
    }
}















