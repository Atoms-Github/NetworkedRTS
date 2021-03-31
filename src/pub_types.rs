use std::collections::HashMap;
use crate::rts::game::game_state::Resources;
use std::sync::Arc;


pub type PlayerID = u32;
pub type PointFloat = nalgebra::Point2<f32>;
pub type HashType = u64;
pub type FrameIndex = usize;
pub type ResourcesPtr = Arc<Resources>;