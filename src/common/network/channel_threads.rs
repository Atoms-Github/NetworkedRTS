use std::io::{Read, Write, Error, ErrorKind};
use std::net::{TcpStream, UdpSocket, SocketAddr, SocketAddrV4, Ipv4Addr};
use std::thread;
use std::time::SystemTime;

use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

use crate::common::gameplay::game::game_state::*;
use crate::common::logic::logic_sim_tailer_seg::*;
use crate::common::time::timekeeping::*;
use crate::common::types::*;
use crate::common::sim_data::sim_data_storage::*;
use std::intrinsics::add_with_overflow;
use crate::common::data::hash_seg::FramedHash;
use crossbeam_channel::*;
use crate::common::utils::util_functions::gen_fake_address;
use crate::common::network::external_msg::ExternalMsg;
use std::sync::mpsc::TryRecvError::Disconnected;
use crate::common::network::channel_threads::SocketIncEvent::{Diconnect, Msg};

pub trait GameSocketTcp{
    fn start_listening(self, msgs_sink: Sender<SocketIncEvent>);
    fn send_msg(&mut self, message: &ExternalMsg);
}
pub enum SocketIncEvent {
    Msg(ExternalMsg, SocketAddr),
    Diconnect(SocketAddr),
}
pub trait GameSocketUdp{
    fn start_listening(self, msgs_sink: Sender<SocketIncEvent>);
    fn send_msg(&self, message: &ExternalMsg, addr: &SocketAddr);
    fn send_msg_to_connected(&self, message: &ExternalMsg);
//    fn start_listening_connected(self, msgs_sink: Sender<(ExternalMsg, SocketAddr)>);
}
impl GameSocketTcp for TcpStream{
    fn start_listening(mut self, msgs_sink: Sender<SocketIncEvent>) {
        thread::Builder::new().name("StreamDeserializerTCP".to_string()).spawn(move ||{
            let peer_address = self.peer_addr().unwrap();

            loop{
                let mut message_buffer = vec![0; 65_535];
                let bytes_read_maybe = self.read(&mut message_buffer);
                match bytes_read_maybe{
                    Result::Err(error) => {
                        log::warn!("Player disconnected. {}", error.to_string());
                        msgs_sink.send(Diconnect(peer_address)).unwrap();
                        return; // Kill thread.
                    }
                    Result::Ok(0) => {
                        log::warn!("Player disconnected. No tcp bytes read.");
                        msgs_sink.send(Diconnect(peer_address)).unwrap();
                        return; // Kill thread
                    }
                    Result::Ok(bytes_read) => {
                        let result = bincode::deserialize::<ExternalMsg>(&message_buffer[..]);
                        match result{
                            Ok(msg) => {
                                if crate::DEBUG_MSGS_NET{
                                    log::debug!("<- {:?}", msg);
                                }
                                msgs_sink.send(Msg(msg, peer_address.clone())).unwrap();
                            }
                            err => {
                                panic!("Err {:?}", err)
                            }
                        }
                    }
                }
            }
        }).unwrap();
    }

    fn send_msg(&mut self, message: &ExternalMsg) {
        let connection_init_bytes = bincode::serialize(message).unwrap();
        self.write_all(&connection_init_bytes).unwrap();
        self.flush().unwrap();

        if crate::DEBUG_MSGS_NET{
            log::debug!("->: {:?}", message);
        }
    }
}
impl GameSocketUdp for UdpSocket{
    fn start_listening(self, msgs_sink: Sender<SocketIncEvent>) {
        thread::Builder::new().name("StreamDeserializerUDP".to_string()).spawn(move ||{
            let mut message_buffer = [0; 65_535];
            loop{
                let new_socket = self.try_clone().unwrap();
                match new_socket.recv_from(&mut message_buffer){
                    Result::Err(error) => {
                        log::warn!("                          ");
                        log::warn!("Failed to receive udp message from someone {:?}", error);
                        log::warn!("                          ");
                    }
                    Result::Ok((0, address)) => {
                        log::warn!("Udp read 0 bytes from {}", address.to_string());
                    }
                    Ok((message_size_bytes, address)) => {
                        let result = bincode::deserialize::<ExternalMsg>(&message_buffer[..]);
                        match result{
                            Ok(msg) => {
                                if crate::DEBUG_MSGS_NET{
                                    log::debug!("<-- {:?}", msg);
                                }
                                msgs_sink.send(Msg(msg, address)).unwrap();
                            }
                            err => {
                                panic!("Err {:?}", err)
                            }
                        }
                    }
                }
            }
        }).unwrap();
    }
    fn send_msg(&self, message: &ExternalMsg, address: &SocketAddr) {
        let msg_buffer = bincode::serialize(message).unwrap();
        self.send_to(&msg_buffer, address).unwrap();

        if crate::DEBUG_MSGS_NET{
            log::debug!("->({}): {:?}", msg_buffer.len(), message);
        }
    }
    fn send_msg_to_connected(&self, message: &ExternalMsg) {
        let msg_buffer = bincode::serialize(message).unwrap();

        self.send(&msg_buffer).unwrap();

        if crate::DEBUG_MSGS_NET{
            log::debug!("->({}): {:?}", msg_buffer.len(), message);
        }
    }
}
pub trait Filterable{
    fn filter_address(self, msg: Option<ExternalMsg>) -> Receiver<ExternalMsg>;
}
impl Filterable for Receiver<SocketIncEvent>{
    fn filter_address(self, msg: Option<ExternalMsg>) -> Receiver<ExternalMsg>{
        let (sink,rec) = unbounded();
        thread::spawn(move ||{
            loop{
                match self.recv(){
                    Ok(SocketIncEvent::Msg(msg, address)) => {
                        sink.send(msg).unwrap();
                    }
                    Ok(item) => {
                        panic!("Not sure how to handle. UDP disconnect.");
                    }
                    Err(error) => {
                        panic!("Not sure how to handle. UDP steam hang up.");
                    }
                }
            }
        });
        return rec;
    }
}

