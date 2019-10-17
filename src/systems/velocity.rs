use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};
use std::any::TypeId;
use crate::systems::position::PositionComp;
use crate::systems::render::RenderComp;
use crate::ecs::world::*;

create_system!( velocity_system | secret_velocity_system
	| my_position:PositionComp, my_velocity:VelocityComp
	|
	|
);

fn velocity_system(d: &mut Data, e: Entity){
	e.my_position(d).x += e.my_velocity(d).x;
	e.my_position(d).y += e.my_velocity(d).y;
}

#[derive(Debug, Clone)]
pub struct VelocityComp {
	pub x: f32,
	pub y: f32,
}