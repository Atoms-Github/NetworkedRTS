use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};
use std::any::TypeId;
use crate::systems::position::PositionComp;
use crate::ecs::world::*;
use serde::{Serialize, Deserialize};



#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct SizeComp {
	pub x: f32,
	pub y: f32,
}