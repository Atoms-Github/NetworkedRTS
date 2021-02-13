use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, Mutex, RwLock, RwLockWriteGuard};
use std::thread;
use std::time::{SystemTime, Duration};

use crate::common::network::external_msg::*;
use crate::common::types::*;
use bimap::BiHashMap;
use std::borrow::Borrow;

use crossbeam_channel::{unbounded, Select};
use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;
use crate::common::data::hash_seg::FramedHash;
use crate::server::net_hub_back_not_seg::{NetHubBackIn, NetHubBackMsgOut, NetHubBackMsgIn};

pub struct NetworkingHubEx {
    pub down_sink: Sender<NetHubFrontMsgIn>,
    pub up_rec: Receiver<NetHubFrontMsgOut>,
}

pub struct NetworkingHubIn {
    host_addr_str: String,
    next_player_id: PlayerID,
    player_id_map: ArcRw<BiHashMap<SocketAddr, PlayerID>>,
}
pub enum NetHubFrontMsgOut{
    NewPlayer(PlayerID),
    PlayerDiscon(PlayerID),
    NewMsg(ExternalMsg, PlayerID)
}
pub enum NetHubFrontMsgIn{
    MsgToSingle(ExternalMsg, PlayerID, /*Reliable*/bool),
    MsgToAllExcept(ExternalMsg, PlayerID, /*Reliable*/bool),
    MsgToAll(ExternalMsg, /*Reliable*/bool),
    DropPlayer(PlayerID)
}
impl NetworkingHubEx{
    pub fn start(host_addr_str: String) -> Self {
        NetworkingHubIn{
            host_addr_str,
            next_player_id: 0,
            player_id_map: Default::default(),
        }.start_hosting()
    }
}
// Manages the server's incoming and outgoing network messages.
impl NetworkingHubIn {
    pub fn start_hosting(mut self) -> NetworkingHubEx{
        let (down_sink, down_rec) = unbounded();
        let (up_sink, up_rec) = unbounded();


        let net_hub_backend = NetHubBackIn::new(self.host_addr_str.clone()).start();

        self.start_handling_upwards(up_sink, net_hub_backend.msg_out);
        self.start_handling_downwards(down_rec, net_hub_backend.msg_in);

        NetworkingHubEx{
            down_sink,
            up_rec,
        }
    }
    fn start_handling_upwards(&self, above_out_sink: Sender<NetHubFrontMsgOut>, back_up_rec: Receiver<NetHubBackMsgOut>){
        let mut next_player_id = 0;
        let players = self.player_id_map.clone();
        thread::spawn(move ||{
            loop{
                let next_msg = back_up_rec.recv().unwrap();
                let mut players_map = players.read().unwrap();
                match next_msg {
                    NetHubBackMsgOut::NewMsg(msg, address) => {
                        let player_id = players_map.get_by_left(&address).unwrap_or_else(|| panic!("Can't find player address {}", address));
                        above_out_sink.send(NetHubFrontMsgOut::NewMsg(msg, *player_id)).unwrap();
                    }
                    NetHubBackMsgOut::NewPlayer(address) => {
                        let mut player_id;
                        let player_id_option = players_map.get_by_left(&address);
                        match player_id_option {
                            Some(existing_player_id) => {
                                player_id = *existing_player_id;
                            }
                            None => {
                                std::mem::drop(players_map);
                                let mut writable_players_map = players.write().unwrap();
                                player_id = next_player_id;
                                next_player_id += 1;
                                log::info!("NEW PLAYER! {} {}", address, player_id);
                                writable_players_map.insert(address, player_id);
                                std::mem::drop(writable_players_map);
                            }
                        }
                        above_out_sink.send(NetHubFrontMsgOut::NewPlayer(player_id)).unwrap();
                    }
                    NetHubBackMsgOut::PlayerDiscon(address) => {
                        let player_id = players_map.get_by_left(&address).unwrap();
                        above_out_sink.send(NetHubFrontMsgOut::PlayerDiscon(*player_id)).unwrap();
                    }
                }
            }
        });
    }
    fn start_handling_downwards(&self, above_in_rec: Receiver<NetHubFrontMsgIn>, back_down_sink: Sender<NetHubBackMsgIn>){
        let players = self.player_id_map.clone();
        thread::spawn(move ||{
            loop{
                let next_msg = above_in_rec.recv().unwrap();
                let players_map = players.read().unwrap();
                match next_msg {
                    NetHubFrontMsgIn::DropPlayer(player_id) => {
                        let address = players_map.get_by_right(&player_id).unwrap();
                        back_down_sink.send(NetHubBackMsgIn::DropPlayer(*address)).unwrap();
                    }
                    NetHubFrontMsgIn::MsgToSingle(msg, player_id, reliable) => {
                        let address = players_map.get_by_right(&player_id).unwrap();
                        back_down_sink.send(NetHubBackMsgIn::SendMsg(*address, msg, reliable)).unwrap();
                    }
                    NetHubFrontMsgIn::MsgToAllExcept(msg, non_player_id, reliable) => {
                        for (address, player_id) in players_map.iter(){
                            if *player_id != non_player_id{
                                back_down_sink.send(NetHubBackMsgIn::SendMsg(*address, msg.clone(), reliable)).unwrap();
                            }
                        }
                    }
                    NetHubFrontMsgIn::MsgToAll(msg, reliable) => {
                        for (address, player_id) in players_map.iter(){
                            back_down_sink.send(NetHubBackMsgIn::SendMsg(*address, msg.clone(), reliable)).unwrap();
                        }
                    }
                }
            }
        });
    }
}








