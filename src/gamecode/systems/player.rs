use std::collections::{BTreeSet};

use serde::{Deserialize, Serialize};

use std::hash::*;
use crate::create_system;
use crate::common::gameplay::ecs::world::*;
use crate::common::gameplay::systems::velocity::VelocityComp;
use crate::common::types::PlayerID;


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



