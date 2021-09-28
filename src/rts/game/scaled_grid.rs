use crate::rts::game::grid::*;
use serde::*;
use serde::de::DeserializeOwned;
use crate::pub_types::{PointFloat, GridBox};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScaledGrid<T>{
    pub grid: Grid<T>,
    pub scale: f32,
}
impl<T: Serialize + DeserializeOwned + Clone + Default> ScaledGrid<T>{
    pub fn new(scale: f32) -> Self{
        Self{
            grid: Grid::new(0,0),
            scale,
        }
    }
    pub fn new_from_grid(grid: Grid<T>, scale: f32) -> Self{
        Self{
            grid,
            scale
        }
    }
    pub fn get_scaled_width(&self) -> f32{
        self.scale * self.grid.len_x() as f32
    }
    pub fn get_scaled_height(&self) -> f32{
        self.scale * self.grid.len_y() as f32
    }
    pub fn get_unwrap(&self, target: &GridBox) -> &T{
        self.grid.get_unwrap(target)
    }
    pub fn get(&self, target: &GridBox) -> Option<&T>{
        self.grid.get(target)
    }
    pub fn get_by_point_unwrap(&self, loc: &PointFloat) -> &T{
        self.get_by_point(loc).unwrap()
    }
    pub fn get_by_point(&self, loc: &PointFloat) -> Option<&T>{
        let b = loc.clone().map(|value|{(value / self.scale) as i32});
        return self.grid.get(&b);
    }
    pub fn get_by_point_mut(&mut self, loc: &PointFloat) -> Option<&mut T>{
        let b = loc.clone().map(|value|{(value / self.scale) as i32});
        return self.grid.get_mut(&b);
    }
    pub fn get_grid_coord(&self, loc: &PointFloat) -> GridBox{
        let b = loc.clone().map(|value|{(value / self.scale) as i32});
        return b;
    }
    pub fn get_box_centre(&self, gridbox: &GridBox) -> PointFloat{
        return PointFloat::new((gridbox.x as f32 + 0.5) * self.scale,
                               (gridbox.y as f32 + 0.5) * self.scale);
    }
    pub fn get_grid_coord_maybe(&self, loc: &PointFloat) -> Option<GridBox>{
        let b = loc.clone().map(|value|{(value / self.scale) as i32});
        if self.grid.is_valid(&b){
            return Some(b);
        }else{
            return None;
        }
    }
}