use std::io::{Read, Write, Error, ErrorKind};
use std::net::{TcpStream, UdpSocket, SocketAddr, SocketAddrV4, Ipv4Addr};
use std::thread;
use std::time::SystemTime;

use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

use crate::netcode::*;
use crate::netcode::common::timekeeping::*;
use std::intrinsics::add_with_overflow;
use crossbeam_channel::*;
use crate::netcode::common::util_functions::gen_fake_address;
use crate::pub_types::{FrameIndex, Shade};
use crate::netcode::client::client_hasher::FramedHash;
use crate::netcode::common::confirmed_data::{SimDataPackage, SimDataQuery};
use crate::netcode::common::net_game_state::NetGameState;

#[derive(Serialize, Deserialize, Clone, Debug)] // Serializing and deserializing enums with data does store which enum it is - we don't need to store the data and enum separately.
pub enum ExternalMsg {
    ConnectionInitQuery,
    ConnectionInitResponse(NetMsgGreetingResponse),
    NewHash(FramedHash),
    GameUpdate(SimDataPackage),
    WorldDownloaded{
        player_name: String,
        color: Shade
    },
    InputQuery(SimDataQuery),
    PingTestQuery(SystemTime),
    PingTestResponse(NetMsgPingTestResponse),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgPingTestResponse{
    pub client_time: SystemTime,
    pub server_time: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgGreetingQuery {

}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgGreetingResponse {
    pub assigned_player_id: PlayerID,
    pub game_state: NetGameState,
    pub known_frame: KnownFrameInfo,
}




