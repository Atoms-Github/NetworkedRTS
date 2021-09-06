use std::collections::HashMap;

use std::sync::Arc;
use nalgebra::U2;
use crate::rts::game::render_resources::RenderResources;
use mint::Point2;


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

pub trait MyPoint{
    fn to_point(&self) -> Point2<f32>;
}
impl MyPoint for PointFloat{
    fn to_point(&self) -> Point2<f32> {
        let e : Point2<f32> = Point2::from([self.x, self.y]);
        return e;
    }
}
