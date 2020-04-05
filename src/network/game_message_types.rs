
use serde::{Deserialize, Serialize};

use crate::network::networking_structs::*;



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewPlayerInfo {
    pub player_id: PlayerID,
    pub frame_added: FrameIndex
}


