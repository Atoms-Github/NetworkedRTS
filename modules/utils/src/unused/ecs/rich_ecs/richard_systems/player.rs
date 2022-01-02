use std::collections::{BTreeSet};

use serde::{Deserialize, Serialize};

use std::hash::*;
use crate::ecs::rich_ecs::system_macro;
use crate::ecs::rich_ecs::world::*;
use crate::rts::systems::velocity::VelocityComp;
use crate::pub_types::PlayerID;


#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct PlayerComp {
	pub player_id: PlayerID,
	pub connected: bool,
}

impl Hash for PlayerComp{
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.player_id.to_be_bytes().hash(state);
	}
}



