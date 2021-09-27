use crate::rts::game::grid::*;
use crate::pub_types::PointFloat;
use crate::rts::game::scaled_grid::ScaledGrid;

use std::collections::VecDeque;

pub struct PathGrinder{

}
impl PathGrinder{
    pub fn new() -> Self{
        Self{

        }
    }
    pub fn pathfind(&mut self, map: &ScaledGrid<bool>, start: PointFloat, end: PointFloat, unit_berth: f32) -> Vec<GridBox>{
        let start = map.get_grid_coord(&start);
        let end = map.get_grid_coord(&end);

        let mut queue = VecDeque::new();
        queue.push_back(start);
        while queue.len() > 0{

        }
        return vec![];
    }
}