
use serde::{Serialize, Deserialize};
use std::fmt;
use crate::network::networking_structs::*;
use crate::players::inputs::*;


#[derive(Serialize, Deserialize, Debug)] // Serializing and deserializing enums with data does store which enum it is - we don't need to store the data and enum separately.
pub enum NetMessageType {
    ConnectionInitQuery(NetMsgConnectionInitQuery),
    InputsUpdate(NetMsgInputsUpdate),
    ConnectionInitResponse(NetMsgConnectionInitResponse),
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
}