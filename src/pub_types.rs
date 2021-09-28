use std::collections::HashMap;

use std::sync::Arc;
use nalgebra::U2;
use crate::rts::game::render_resources::RenderResources;
use mint::Point2;
use ggez::graphics::Color;
use serde::*;


pub type PlayerID = u32;
pub type PointFloat = nalgebra::VectorN<f32, U2>;
pub type HashType = u64;
pub type FrameIndex = usize;
pub type RenderResourcesPtr = Arc<RenderResources>;
pub type GridBox = nalgebra::Vector2<i32>;

pub struct SimMetadata{
    pub delta: f32,
    pub quality: SimQuality,
    pub frame_index: FrameIndex,
}

#[derive(PartialEq)]
pub enum SimQuality{
    DETERMA,
    HEAD
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Shade(pub f32, pub f32, pub f32);

impl Shade{
    pub fn to_color(&self) -> Color{
        Color{
            r: self.0,
            g: self.1,
            b: self.2,
            a: 1.0
        }
    }
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

pub trait MyGridBox{
    fn left(&self) -> GridBox;
    fn right(&self) -> GridBox;
    fn up(&self) -> GridBox;
    fn down(&self) -> GridBox;
}
impl MyGridBox for GridBox{
    fn left(&self) -> GridBox{
        return GridBox::new(self.x - 1, self.y);
    }
    fn right(&self) -> GridBox{
        return GridBox::new(self.x + 1, self.y);
    }
    fn up(&self) -> GridBox{
        return GridBox::new(self.x, self.y - 1);
    }
    fn down(&self) -> GridBox{
        return GridBox::new(self.x, self.y + 1);
    }
}