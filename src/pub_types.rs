use std::collections::HashMap;
use crate::rts::game::game_state::GameResources;
use std::sync::Arc;
use nalgebra::U2;


pub type PlayerID = u32;
pub type PointFloat = nalgebra::VectorN<f32, U2>;
pub type HashType = u64;
pub type FrameIndex = usize;
pub type ResourcesPtr = Arc<GameResources>;

#[derive(PartialEq)]
pub enum SimQuality{
    DETERMA,
    HEAD
}