
use serde::{Deserialize, Serialize};

use crate::network::networking_structs::*;
use crate::players::inputs::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameMessageType {
    InputsUpdate(InputsUpdateInfo),
    NewPlayer(NewPlayerInfo)
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputsUpdateInfo {
    pub player_id: PlayerID,
    pub frame_index: FrameIndex,
    pub input_states: [InputState; 20],
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewPlayerInfo {
    pub player_id: PlayerID,
    pub frame_added: usize
}
