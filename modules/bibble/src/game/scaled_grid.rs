use bibble::::grid::*;
use serde::*;
use serde::de::DeserializeOwned;
use netcode::*;
use game::pub_types::{PointFloat, GridBox};

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
    pub fn get_grid_coord_maybe(&self, loc: &PointFloat) -> Option<GridBox> {
        let b = loc.clone().map(|value| { (value / self.scale) as i32 });
        if self.grid.is_valid(&b) {
            return Some(b);
        } else {
            return None;
        }
    }
}
impl ScaledGrid<bool>{
    pub fn line_all_true(&self, line_start: &PointFloat, line_end: &PointFloat) -> bool{
        let start = line_start.map(|c|{ (c / self.scale) as isize });
        let end = line_end.map(|c|{ (c / self.scale) as isize });
        for (x, y) in bresenham::Bresenham::new((start.x, start.y), (end.x, end.y)) {
            let gridbox = GridBox::new(x as i32, y as i32);
            if let Some(can_path) = self.get(&gridbox){
                if !can_path{
                    return false;
                }
            }
        }
        // This Bresenham doesn't do end, so manually add end.
        let gridbox_end = GridBox::new(end.x as i32, end.y as i32);
        if let Some(can_path) = self.get(&gridbox_end){
            if !can_path{
                return false;
            }
        }
        return true;
    }
}
