
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct SizeComp {
	pub x: f32,
	pub y: f32,
}