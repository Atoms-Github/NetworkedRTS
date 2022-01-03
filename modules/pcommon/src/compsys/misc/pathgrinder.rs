use crate::*;
use serde::*;

use std::collections::{VecDeque, HashSet, HashMap};
use std::ops::Div;
use bresenham::Bresenham;


trait MyGridBoxNeighbours{
    fn get_pathable_neighbours(&self, grid: &Grid<bool>) -> Vec<GridBox>;
}
impl MyGridBoxNeighbours for GridBox{
    fn get_pathable_neighbours(&self, grid: &Grid<bool>) -> Vec<GridBox>{
        let potential_neighbours = vec![self.left(), self.right(), self.up(), self.down()];
        let mut neighbours = vec![];
        for neighbour in potential_neighbours{
            let value = grid.get(&neighbour);
            if value == Some(&true){
                neighbours.push(neighbour);
            }
        }
        return neighbours;
        // let top_left = GridBox::new(self.x - 1, self.y - 1);
        // let bottom_right = GridBox::new(self.x + 1, self.y + 1);
        // let mut neighbours = vec![];
        // for (potential_neighbour, value) in grid.iter_square(top_left, bottom_right){
        //     if *value && potential_neighbour != *self{
        //         neighbours.push(potential_neighbour);
        //     }
        // }
        // return neighbours;
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
        let mut path = PathGrinder::grid_pathfind(grid, start, end);

        match path{
            Some(mut path) => {
                path.insert(0, start_pos);
                path.push(end_pos);
                PathGrinder::take_shortcuts(&mut path, grid);
                path.remove(0);
                return path;
            }
            None => { // No route.
                return vec![end_pos];
            }
        }
    }
    pub fn take_shortcuts(path: &mut Vec<PointFloat>, grid: &ScaledGrid<bool>) {
        let mut start_index = 0;
        loop{
            if let Some(startpos) = path.get(start_index){
                let end_index = start_index + 2;
                if let Some(endpos) = path.get(end_index){
                    if grid.line_all_true(startpos, endpos){
                        // Can do shortcut. Remove useless node, and go again from where you are.
                        path.remove(start_index + 1);
                    }else{
                        // Can't do shortcut. Try next one along.
                        start_index += 1;
                    }
                }else{
                    return;
                }
            }else{
                return;
            }
        }
    }
        /// Returns a list of all the places you need to step to get from start to end. (So includes start and end).
    fn grid_pathfind(grid: &ScaledGrid<bool>, start: GridBox, end: GridBox) -> Option<Vec<PointFloat>> {
        if start == end {
            return Some(vec![grid.get_box_centre(&end)]);
        }

        // Includes source box too.
        let mut visited_boxes: HashMap<GridBox, GridBox> = HashMap::new();
        let mut open_boxes: VecDeque<(GridBox, GridBox)> = VecDeque::new();
        open_boxes.push_back((start.clone() as GridBox, start));
        visited_boxes.insert(start.clone() as GridBox, start.clone() as GridBox);

        while open_boxes.len() > 0 {
            let (expanding_box, from_box) = open_boxes.pop_front().unwrap();

            for neighbour in expanding_box.get_pathable_neighbours(&grid.grid) {
                if !visited_boxes.contains_key(&neighbour) {
                    open_boxes.push_back((neighbour.clone(), expanding_box.clone() as GridBox));
                    visited_boxes.insert(neighbour.clone() as GridBox, expanding_box.clone() as GridBox);
                }
                if neighbour == end {
                    // Now reconstruct the path.
                    let mut path = backtrack_path(end, grid, visited_boxes);
                    return Some(path);
                }
            }
        }
        return None;
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
        path_grid.push( grid.get_box_centre(one_step_back));
        end = one_step_back.clone() as GridBox;
    }
    path_grid.reverse();
    return path_grid;
}
#[test]
fn test_inverse_wp() {
    let bi = Bresenham::new((0, 1), (0, 2));
    let res: Vec<_> = bi.collect();

    assert_eq!(res, [(0,1)])
}