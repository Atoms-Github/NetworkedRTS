
use serde::{Deserialize, Serialize};

use crate::network::networking_structs::*;
use crate::players::inputs::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogicInwardsMessage {
    InputsUpdate(PlayerInputsSegmentResponse),
    NewPlayer(NewPlayerInfo)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewPlayerInfo {
    pub player_id: PlayerID,
    pub frame_added: FrameIndex
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogicOutwardsMessage {
    PlayerInputsNeeded(PlayerInputsSegmentRequest)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerInputsSegmentRequest {
    pub player_id: PlayerID,
    pub start_frame: FrameIndex,
    pub number_of_frames: usize // Usually 20.
}
