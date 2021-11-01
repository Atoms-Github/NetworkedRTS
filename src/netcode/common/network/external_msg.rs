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
use crate::netcode::common::sim_data::net_game_state::{NetPlayerProperty, NetGameState};
use crate::pub_types::{FrameIndex, Shade};

#[derive(Serialize, Deserialize, Clone, Debug)] // Serializing and deserializing enums with data does store which enum it is - we don't need to store the data and enum separately.
pub enum ExternalMsg {
    ConnectionInitQuery(NetMsgGreetingQuery),
    ConnectionInitResponse(NetMsgGreetingResponse),
    NewHash(FramedHash),
    GameUpdate(SimDataPackage),
    WorldDownloaded(WorldDownloadedInfo),
    InputQuery(SimDataQuery),
    PingTestQuery(SystemTime),
    PingTestResponse(NetMsgPingTestResponse),
    HelloDebug(),
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
pub struct WorldDownloadedInfo{
    pub player_name: String,
    pub color: Shade
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





