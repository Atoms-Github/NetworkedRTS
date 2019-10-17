
use serde::{Serialize, Deserialize};
use std::fmt;


#[derive(Serialize, Deserialize, Debug)] // Serializing and deserializing enums with data does store which enum it is - we don't need to store the data and enum separately.
pub enum NetMessageType {
    ConnectionInitQuery(NetMsgConnectionInitQuery),
    InputsUpdate(NetMsgInputsUpdate),
    ConnectionInitResponse(NetMsgConnectionInitResponse),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NetMsgInputsUpdate{
    pub controllers: Vec<PlayerController>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetMsgConnectionInitQuery {
    pub welcome_msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetMsgConnectionInitResponse {
    pub assigned_player_id: PlayerID,
}