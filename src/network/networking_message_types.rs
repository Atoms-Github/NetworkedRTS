
use serde::{Serialize, Deserialize};
use std::fmt;
use crate::network::networking_structs::*;
use crate::players::inputs::*;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Serialize, Deserialize, Debug)] // Serializing and deserializing enums with data does store which enum it is - we don't need to store the data and enum separately.
pub enum NetMessageType {
    ConnectionInitQuery(NetMsgConnectionInitQuery),
    InputsUpdate(NetMsgInputsUpdate),
    ConnectionInitResponse(NetMsgConnectionInitResponse),
    LocalCommand(LocalCommandInfo)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalCommandInfo{
    pub command: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetMsgInputsUpdate{
    pub player_id: PlayerID,
    pub frame_index: FrameIndex,
    pub input_states: [InputState; 20],
}



#[derive(Serialize, Deserialize, Debug)]
pub struct NetMsgConnectionInitQuery {
    pub my_player_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetMsgConnectionInitResponse {
    pub assigned_player_id: PlayerID,
    pub game_state: GameState,
    pub frames_gathered_so_far: FramesStoragePartial,
    pub server_time_of_state: SystemTime
}