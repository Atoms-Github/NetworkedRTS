use std::collections::{BTreeSet};

use serde::{Deserialize, Serialize};

use std::hash::*;
use crate::gamecode::ecs::system_macro;
use crate::gamecode::ecs::world::*;
use crate::gamecode::systems::velocity::VelocityComp;

create_system!( position_system | secret_position_system
	| my_position: PositionComp, my_velocity: VelocityComp
	|
	|
);

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct PositionComp {
	pub x: f32,
	pub y: f32,
}

impl Hash for PositionComp{
	fn hash<H: Hasher>(&self, state: &mut H) { // Can fix with fixed and/or cordick first.
		self.x.to_be_bytes().hash(state);
		self.y.to_be_bytes().hash(state);
	}
}



fn position_system(d: &mut Data, e: Entity) {

}



