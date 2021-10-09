use std::io::{Read, Write, Error, ErrorKind};
use std::net::{TcpStream, UdpSocket, SocketAddr, SocketAddrV4, Ipv4Addr};
use std::thread;
use std::time::SystemTime;

use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

use crate::netcode::*;
use crate::netcode::common::logic::logic_sim_tailer_seg::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::common::sim_data::sim_data_storage::*;
use std::intrinsics::add_with_overflow;
use crate::netcode::common::logic::logic_sim_tailer_seg::FramedHash;
use crossbeam_channel::*;
use crate::netcode::common::utils::util_functions::gen_fake_address;
use crate::netcode::common::network::external_msg::ExternalMsg;
use std::sync::mpsc::TryRecvError::Disconnected;
use crate::netcode::common::network::channel_threads::SocketIncEvent::{Diconnect, Msg};
use crate::netcode::netcode_types::*;
use crate::pub_types::*;

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
                // First 4 bytes are content's size.
                let mut message_size_buffer = vec![0; 4];
                let message_size_peek_maybe = self.read_exact(&mut message_size_buffer);
                match message_size_peek_maybe {
                    Result::Err(error) => {
                        log::warn!("Player disconnected. {}", error.to_string());
                        msgs_sink.send(Diconnect(peer_address)).unwrap();
                        return; // Kill thread.
                    }
                    Result::Ok(()) => {
                        // Should've read all 4 for size.
                        let content_size = bincode::deserialize::<u32>(&message_size_buffer).unwrap() as usize;
                        let mut message_buffer_compressed = vec![0; content_size];
                        let content_read_size = self.read_exact(&mut message_buffer_compressed).unwrap();
                        let message_decompressed = crate::utils::decompress(message_buffer_compressed);

                        let content_deser_result = bincode::deserialize::<ExternalMsg>(&message_decompressed[..]);
                        match content_deser_result {
                            Ok(msg) => {
                                if crate::DEBUG_MSGS_NET{
                                    log::debug!("<--u {:?}", msg);
                                }
                                msgs_sink.send(Msg(msg, peer_address.clone())).unwrap();
                            }
                            err => {
                                panic!("Deserialize error: {:?}", err)
                            }
                        }
                    }
                }
            }
        }).unwrap();
    }

    fn send_msg(&mut self, message: &ExternalMsg){
        if crate::DEBUG_MSGS_NET{
            log::debug!("-->t: {:?}", message);
        }
        let mut compressed_contents_bytes = crate::utils::compress(bincode::serialize(message).unwrap());

        let message_size : u32 = compressed_contents_bytes.len() as u32;
        // Prepend message size.
        let mut message_wire_bytes = bincode::serialize(&message_size).unwrap();
        message_wire_bytes.append(&mut compressed_contents_bytes);
        self.write_all(&message_wire_bytes).unwrap();
        self.flush().unwrap();
    }
}


impl GameSocketUdp for UdpSocket{
    fn start_listening(self, msgs_sink: Sender<SocketIncEvent>) {
        thread::Builder::new().name("StreamDeserializerUDP".to_string()).spawn(move ||{
            let mut message_buffer = [0; 65_535];
            loop{
                match self.recv_from(&mut message_buffer){
                    Result::Err(error) => {
                        log::warn!("Did someone disconnect recently? Failed to receive udp message from someone {:?}", error);
                    }
                    Result::Ok((0, address)) => {
                        log::warn!("Udp read 0 bytes from {}", address.to_string());
                    }
                    Ok((message_size_bytes, address)) => {
                        let result = bincode::deserialize::<ExternalMsg>(&message_buffer[..]);
                        match result{
                            Ok(msg) => {
                                if crate::DEBUG_MSGS_NET{
                                    log::debug!("<--u {:?}", msg);
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
            log::debug!("-->u({}): {:?}", msg_buffer.len(), message);
        }
    }
    fn send_msg_to_connected(&self, message: &ExternalMsg) {
        let msg_buffer = bincode::serialize(message).unwrap();

        self.send(&msg_buffer).unwrap();

        if crate::DEBUG_MSGS_NET{
            log::debug!("-->uc({}): {:?}", msg_buffer.len(), message);
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

