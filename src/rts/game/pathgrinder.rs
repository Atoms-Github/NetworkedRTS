use crate::rts::game::grid::*;
use crate::pub_types::{PointFloat, GridBox};
use crate::rts::game::scaled_grid::ScaledGrid;
use serde::*;

use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathGrinder{

}
impl PathGrinder{
    pub fn new() -> Self{
        Self{

        }
    }
    pub fn pathfind(&mut self, map: &ScaledGrid<bool>, start_pos: PointFloat, end_pos: PointFloat, unit_berth: f32) -> Vec<PointFloat>{
        // We can assume the same map will always be passed in.
        // Its up to the caller to recreate us if they want to pass something different in.

        let start = map.get_grid_coord(&start_pos);
        let end = map.get_grid_coord(&end_pos);

        let mut queue = VecDeque::new();
        queue.push_back(start);
        return vec![end_pos];
    }
}