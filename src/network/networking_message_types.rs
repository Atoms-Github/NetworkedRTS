
use serde::{Serialize, Deserialize};
use std::fmt;
use crate::network::networking_structs::*;
use crate::players::inputs::*;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Serialize, Deserialize, Clone, Debug)] // Serializing and deserializing enums with data does store which enum it is - we don't need to store the data and enum separately.
pub enum NetMessageType {
    ConnectionInitQuery(NetMsgConnectionInitQuery),
    InputsUpdate(NetMsgInputsUpdate),
    ConnectionInitResponse(NetMsgConnectionInitResponse),
    LocalCommand(LocalCommandInfo),
    NewPlayer(NetMsgNewPlayer)
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgNewPlayer{
    pub player_id: PlayerID,
    pub frame_added: usize
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalCommandInfo{
    pub command: String
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgInputsUpdate{
    pub player_id: PlayerID,
    pub frame_index: FrameIndex,
    pub input_states: [InputState; 20],
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgConnectionInitQuery {
    pub my_player_name: String,
    pub test_field: String,
    pub test_two: i64
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetMsgConnectionInitResponse {
    pub assigned_player_id: PlayerID,
    pub game_state: GameState,
    pub frames_gathered_so_far: FramesStoragePartial,
    pub known_frame_info: KnownFrameInfo
}