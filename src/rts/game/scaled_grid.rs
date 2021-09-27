use crate::rts::game::grid::*;
use serde::*;
use serde::de::DeserializeOwned;
use crate::pub_types::PointFloat;

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub fn new_from_existing<N, E>(existing: &ScaledGrid<E>) -> Self{
        let grid = Grid::new(existing.grid.len_x(), existing.grid.len_y());
        Self{
            grid,
            scale: existing.scale
        }
    }
    pub fn get_by_point(&self, loc: &PointFloat) -> &T{
        let b = loc.clone().map(|value|{(value / self.scale) as i32});
        return self.grid.get_unwrap(&b);
    }
    pub fn get_grid_coord(&self, loc: &PointFloat) -> GridBox{
        let b = loc.clone().map(|value|{(value / self.scale) as i32});
        return b;
    }
}