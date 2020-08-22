use std::collections::{BTreeSet};
use crate::common::gameplay::systems::position::PositionComp;
use crate::common::gameplay::ecs::world::*;
use crate::create_system;
use serde::{Serialize, Deserialize};
use std::hash::*;


create_system!( velocity_system | secret_velocity_system
	| my_position:PositionComp, my_velocity:VelocityComp
	|
	|
);

fn velocity_system(d: &mut Data, e: Entity){
	
	e.my_position(d).x += e.my_velocity(d).x;
	e.my_position(d).y += e.my_velocity(d).y;

}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct VelocityComp {
	pub x: f32,
	pub y: f32,
}

impl Hash for VelocityComp{
	fn hash<H: Hasher>(&self, state: &mut H) { // Can fix with fixed and/or cordick first.
		self.x.to_be_bytes().hash(state);
		self.y.to_be_bytes().hash(state);
	}
}
