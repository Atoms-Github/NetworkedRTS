use std::io::{Read, Write, Error, ErrorKind};
use std::net::{TcpStream, UdpSocket, SocketAddr, SocketAddrV4, Ipv4Addr};
use std::thread;
use std::time::SystemTime;

use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

use crate::*;
use crate::common::timekeeping::*;
use crossbeam_channel::*;
use crate::pub_types::{FrameIndex, Shade};
use crate::client::client_hasher::FramedHash;
use crate::common::confirmed_data::{SimDataPackage, SimDataQuery};
use crate::common::net_game_state::{NetGameState, GameState};
use std::fmt::{Debug, Formatter};

#[derive(Serialize, Deserialize, Clone, Debug)] // Serializing and deserializing enums with data does store which enum it is - we don't need to store the data and enum separately.
pub enum ExternalMsg<T>{
    ConnectionInitQuery,
    ConnectionInitResponse(NetMsgGreetingResponse<T>),
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
#[derive(Serialize, Deserialize, Clone)]
pub struct NetMsgGreetingResponse<T> {
    pub assigned_player_id: PlayerID,
    pub game_state: NetGameState<T>,
    pub known_frame: KnownFrameInfo,
}
impl<T : Debug> Debug for NetMsgGreetingResponse<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ANetMsgGreetingResponse").finish()
    }
}





