use std::collections::HashMap;

use std::sync::Arc;
use nalgebra::U2;
use mint::Point2;
use ggez::graphics::Color;
use serde::*;
pub use netcode::*;





pub type PointFloat = nalgebra::VectorN<f32, U2>;
pub type PointInt = nalgebra::VectorN<i32, U2>;
pub type GridBox = nalgebra::Vector2<i32>;





pub trait MyPoint{
    fn to_point(&self) -> Point2<f32>;
    fn to_ggez_rect(&self, size: &PointFloat) -> ggez::graphics::Rect;
    fn dist(&self, other: &PointFloat) -> f32;
}
impl MyPoint for PointFloat{
    fn to_point(&self) -> Point2<f32> {
        let e : Point2<f32> = Point2::from([self.x, self.y]);
        return e;
    }
    fn to_ggez_rect(&self, size: &PointFloat) -> ggez::graphics::Rect{
        return ggez::graphics::Rect::new(self.x - size.x / 2.0, self.y - size.y / 2.0, size.x, size.y);
    }

    fn dist(&self, other: &PointFloat) -> f32 {
        return ((other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)).sqrt();
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