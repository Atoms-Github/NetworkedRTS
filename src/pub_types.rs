use std::collections::HashMap;

use std::sync::Arc;
use nalgebra::U2;
use crate::rts::game::render_resources::RenderResources;


pub type PlayerID = u32;
pub type PointFloat = nalgebra::VectorN<f32, U2>;
pub type HashType = u64;
pub type FrameIndex = usize;
pub type RenderResourcesPtr = Arc<RenderResources>;

#[derive(PartialEq)]
pub enum SimQuality{
    DETERMA,
    HEAD
}