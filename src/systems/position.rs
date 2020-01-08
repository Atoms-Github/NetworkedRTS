use std::collections::{BTreeSet};

use serde::{Deserialize, Serialize};

use crate::create_system;
use crate::ecs::system_macro::*;
use crate::ecs::world::*;
use crate::systems::velocity::VelocityComp;

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



fn position_system(d: &mut Data, e: Entity) {
	let test = TestWoah{
		field: 0
	};

}
































