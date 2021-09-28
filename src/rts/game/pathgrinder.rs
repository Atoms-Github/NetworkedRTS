use crate::rts::game::grid::*;
use crate::pub_types::{PointFloat, GridBox};
use crate::rts::game::scaled_grid::ScaledGrid;
use serde::*;

use std::collections::{VecDeque, HashSet, HashMap};
use std::ops::Div;

trait MyGridBox{
    fn get_pathable_neighbours(&self, grid: &Grid<bool>) -> Vec<GridBox>;
}
impl MyGridBox for GridBox{
    fn get_pathable_neighbours(&self, grid: &Grid<bool>) -> Vec<GridBox>{
        let top_left = GridBox::new(self.x - 1, self.y - 1);
        let bottom_right = GridBox::new(self.x + 1, self.y + 1);
        let mut neighbours = vec![];
        for (potential_neighbour, value) in grid.iter_square(top_left, bottom_right){
            if *value && potential_neighbour != *self{
                neighbours.push(potential_neighbour);
            }
        }
        return neighbours;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathGrinder{

}
impl PathGrinder{
    pub fn new() -> Self{
        Self{

        }
    }
    pub fn pathfind(&mut self, grid: &ScaledGrid<bool>, start_pos: PointFloat, end_pos: PointFloat, unit_berth: f32) -> Vec<PointFloat>{
        // We can assume the same map will always be passed in.
        // Its up to the caller to recreate us if they want to pass something different in.

        let start = grid.get_grid_coord(&start_pos);
        let end = grid.get_grid_coord(&end_pos);

        // Includes source box too.
        let mut visited_boxes : HashMap<GridBox, GridBox> = HashMap::new();
        let mut open_boxes : VecDeque<(GridBox, GridBox)> = VecDeque::new();
        open_boxes.push_back((start.clone() as GridBox, start));

        while open_boxes.len() > 0{
            let (expanding_box, from_box) = open_boxes.pop_front().unwrap();
            if !visited_boxes.contains_key(&expanding_box){
                visited_boxes.insert(expanding_box.clone() as GridBox, from_box);
            }
            for neighbour in expanding_box.get_pathable_neighbours(&grid.grid){
                if neighbour == end{
                    visited_boxes.insert(neighbour as GridBox, expanding_box);
                    // Now reconstruct the path.
                    let mut path = backtrack_path(end, grid, visited_boxes);
                    path.push(end_pos);
                    return path;
                }
                if !visited_boxes.contains_key(&neighbour){
                    open_boxes.push_back((neighbour, expanding_box.clone() as GridBox));
                }
            }
        }
        println!("NoPath");
        return vec![end_pos];
    }
}
fn backtrack_path(end: GridBox, grid: &ScaledGrid<bool>, visited_boxes : HashMap<GridBox, GridBox>) -> Vec<PointFloat>{
    let mut path_grid = vec![];
    let mut end = end;
    loop{
        let one_step_back = visited_boxes.get(&end).expect("How did we get to a point which couldn't be traced back?");
        if *one_step_back == end{ // Reached the start. No where else to go.
            break;
        }
        path_grid.push(one_step_back.clone() as GridBox);
        end = one_step_back.clone() as GridBox;
    }

    // Now convert vec of grid boxes into positions.
    let mut path_points = vec![];
    for gridbox in path_grid{
        path_points.push(grid.get_box_centre(&gridbox));
    }
    path_points.reverse();
    return path_points;
}