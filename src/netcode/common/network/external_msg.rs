use std::io::{Read, Write, Error, ErrorKind};
use std::net::{TcpStream, UdpSocket, SocketAddr, SocketAddrV4, Ipv4Addr};
use std::thread;
use std::time::SystemTime;

use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

use crate::netcode::common::gameplay::game::game_state::*;
use crate::netcode::common::logic::logic_sim_tailer_seg::*;
use crate::netcode::common::time::timekeeping::*;
use crate::netcode::common::types::*;
use crate::netcode::common::sim_data::sim_data_storage::*;
use std::intrinsics::add_with_overflow;
use crate::netcode::common::logic::hash_seg::FramedHash;
use crossbeam_channel::*;
use crate::netcode::common::utils::util_functions::gen_fake_address;

#[derive(Serialize, Deserialize, Clone, Debug)] // Serializing and deserializing enums with data does store which enum it is - we don't need to store the data and enum separately.
pub enum ExternalMsg {
    ConnectionInitQuery(NetMsgGreetingQuery),
    ConnectionInitResponse(NetMsgGreetingResponse),
    NewHash(FramedHash),
    GameUpdate(OwnedSimData),
    InputQuery(QuerySimData),
    PingTestQuery(SystemTime),
    PingTestResponse(NetMsgPingTestResponse),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgPingTestResponse{
    pub client_time: SystemTime,
    pub server_time: SystemTime,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalCommandInfo{
    pub command: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgGreetingQuery {
    pub my_player_name: String,
    pub preferred_id: i32,
    pub udp_port: u16,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgGreetingResponse {
    pub assigned_player_id: PlayerID,
    pub game_state: GameState,
    pub players_in_state: Vec<PlayerID>,
    pub known_frame: KnownFrameInfo,
}






