use crate::rts::game::grid::*;
use serde::*;
use serde::de::DeserializeOwned;
use crate::pub_types::PointFloat;

pub struct ScaledGrid<T>{
    pub grid: Grid<T>,
    pub scale: f32,
}
impl<T: Serialize + DeserializeOwned + Clone + Default> ScaledGrid<T>{
    pub fn new(grid: Grid<T>, scale: f32) -> Self{
        Self{
            grid,
            scale
        }
    }
    pub fn get_by_point(&self, loc: &PointFloat) -> &T{
        let b = loc.clone().map(|value|{(value / self.scale) as i32});
        return self.grid.get_unwrap(&b);
    }
}