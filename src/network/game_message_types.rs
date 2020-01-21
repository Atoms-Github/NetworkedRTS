
use serde::{Deserialize, Serialize};

use crate::network::networking_structs::*;
use crate::players::inputs::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogicInwardsMessage {
    InputsUpdate(LogicInputsResponse),
    BonusMsgsUpdate(BonusMsgsResponse)

}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogicInputsResponse { // TODO2: Rename graphical_segment etc to graphical_module.
    pub player_id: PlayerID,
    pub start_frame_index: FrameIndex,
    pub input_states: Vec<PlayerInputSegmentType>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BonusMsgsResponse {
    pub start_frame_index: FrameIndex,
    pub event_lists: Vec<Vec<BonusEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewPlayerInfo {
    pub player_id: PlayerID,
    pub frame_added: FrameIndex
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogicOutwardsMessage {
    InputsNeeded(LogicInfoRequest)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogicInfoRequestType {
    PlayerInputs(PlayerID),
    BonusEvents,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogicInfoRequest {
    pub start_frame: FrameIndex,
    pub number_of_frames: usize, // Usually 20.
    pub type_needed: LogicInfoRequestType,
}
