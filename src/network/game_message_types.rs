
use serde::{Serialize, Deserialize};
use std::{fmt, thread};
use crate::network::networking_structs::*;
use crate::players::inputs::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::mpsc::{Receiver, Sender, channel};
use bytes::Reader;
use std::io::{Read, Write};
use byteorder::ByteOrder;
use std::net::TcpStream;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameMessageType {
    InputsUpdate(GameInputsUpdate),
    NewPlayer(GameMsgNewPlayer)
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameInputsUpdate {
    pub player_id: PlayerID,
    pub frame_index: FrameIndex,
    pub input_states: [InputState; 20],
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameMsgNewPlayer {
    pub player_id: PlayerID,
    pub frame_added: usize
}
