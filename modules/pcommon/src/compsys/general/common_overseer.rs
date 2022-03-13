use netcode::common::net_game_state::StaticFrameData;
use crate::*;

pub const OVERSEER_ENT_ID : GlobalEntityID = 0;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct CommonOverseer {
    pub connected_players: usize,
}
