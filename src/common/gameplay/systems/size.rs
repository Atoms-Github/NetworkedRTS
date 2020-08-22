
use serde::{Deserialize, Serialize};
use std::hash::*;


#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct SizeComp {
	pub x: f32,
	pub y: f32,
}

impl Hash for SizeComp{
	fn hash<H: Hasher>(&self, state: &mut H) { // Can fix with fixed and/or cordick first.
		self.x.to_be_bytes().hash(state);
		self.y.to_be_bytes().hash(state);
	}
}
