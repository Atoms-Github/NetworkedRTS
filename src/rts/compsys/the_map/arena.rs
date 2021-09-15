
use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};

use ggez::event::MouseButton;
use image::Pixel;
use mopa::Any;
use serde::*;
use ggez::graphics::Color;

pub const ARENA_PLOT_SIZE: f32 = 50.0;
pub const PERFORMANCE_MAP_BOX_SIZE: f32 = 100.0;
pub type PathingMap = Vec<Vec<PlotFlooring>>;
pub type PerformanceMap = Vec<Vec<Vec<GlobalEntityID>>>;


pub type Plot = nalgebra::Point2<usize>; // A Plot is a valid box on the map.
pub type PlotSize = nalgebra::Point2<u8>;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PlotFlooring {
    WALL,
    PATH,
    GREEN_RESOURCE,
    STRUCTURE,
}
impl PlotFlooring{
    pub fn get_color(&self) -> Shade{
        match self {
            PlotFlooring::WALL => {
                Shade(0.2, 0.2, 0.2)
            }
            PlotFlooring::PATH => {
                Shade(0.2, 0.5, 0.2)
            }
            PlotFlooring::GREEN_RESOURCE => {
                Shade(0.2, 0.8, 0.2)
            }
            PlotFlooring::STRUCTURE => {
                Shade(0.5,0.2,0.2)
            }
        }
    }
    pub fn can_walk_over(&self) -> bool{
        return !(*self == PlotFlooring::WALL || *self == PlotFlooring::STRUCTURE);
    }
    pub fn from_color(color: Color) -> Self{
        let (r,g,b) = color.to_rgb();
        match (r,g,b){
            (255,255,255) => {Self::PATH}
            (0,255,0) => {Self::GREEN_RESOURCE}
            (0,0,0) => {Self::WALL}
            (_, _, _) => {Self::PATH}
        }
    }
}
impl Default for PlotFlooring {
    fn default() -> Self {
        return Self::PATH;
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArenaComp {
    pub flooring: PathingMap,
    pub performance_map: PerformanceMap,
}


impl ArenaComp {
    pub fn clear_performance_map(&mut self){
        self.performance_map = Self::get_blank_performance_map(&self.flooring);
    }
    pub fn get_nearby_performance_map_entities(&self, position: &PointFloat, radius: f32) -> impl Iterator<Item = &GlobalEntityID>{
        // Steps:
        // Get the radius.
        // Round it up to the next box.
        // That's how many out you want to go, not including yourself, in each direction.
        let mut matching_boxes = vec![];
        let out_in_each_direction = (radius / PERFORMANCE_MAP_BOX_SIZE) as i32 + 1;
        for x_offset in -out_in_each_direction..out_in_each_direction + 1{
            for y_offset in -out_in_each_direction..out_in_each_direction + 1{
                let box_x = (position.x / PERFORMANCE_MAP_BOX_SIZE) as i32 + x_offset;
                let box_y = (position.y / PERFORMANCE_MAP_BOX_SIZE) as i32 + y_offset;
                if box_x >= 0 && box_x < self.performance_map.len() as i32{
                    let column = self.performance_map.get(box_x as usize).unwrap();
                    if box_y >= 0 && box_y < column.len() as i32{
                        let mut square = column.get(box_y as usize).unwrap();
                        matching_boxes.push(square);
                    }
                }
            }
        }
        let iter = matching_boxes.into_iter().flat_map(|f|{f}).into_iter();
        return iter;
    }
    pub fn register_performance_map_entity(&mut self, entity: GlobalEntityID, position: &PointFloat){
        let x = (position.x / PERFORMANCE_MAP_BOX_SIZE) as usize;
        let y = (position.y / PERFORMANCE_MAP_BOX_SIZE) as usize;
        self.performance_map.get_mut(x).unwrap().get_mut(y).unwrap().push(entity);
    }
    pub fn is_box_walkable(&self, x: i32, y: i32) -> bool{ // This could be replaced by a more plotty sort.
        if x >= 0 && x < self.flooring.len() as i32
            && y >= 0 && y < self.flooring.get(0).unwrap().len() as i32{
            let floor = *self.flooring.get(x as usize).unwrap().get(y as usize).unwrap();
            return floor.can_walk_over();
        }else{
            // Out map = in wall.
            return true;
        }
    }
    pub fn get_plot(&self, position: &PointFloat) -> Option<Plot>{
        let square_coords_x = (position.x / ARENA_PLOT_SIZE).floor() as i32;
        let square_coords_y = (position.y / ARENA_PLOT_SIZE).floor() as i32;
        self.get_ploti(square_coords_x, square_coords_x)
    }
    pub fn get_ploti(&self, posx: i32, posy: i32) -> Option<Plot>{
        if posx >= 0 && posx < self.flooring.len() as i32
            && posy >= 0 && posy < self.flooring.get(0).unwrap().len() as i32{
            return Some(Plot::new(posx as usize, posy as usize));
        }else{
            return None;
        }
    }
    // pub fn pos_in_wall(&self, position: &PointFloat) -> bool{
    //     if let Some(plot) = self.get_plot(position){
    //         return self.plot_in_wall(&plot);
    //     }else{ // Off map = in wall.
    //         return true;
    //     }
    // }
    // pub fn plot_in_wall(&self, plot: &Plot) -> bool{
    //     return *self.pathing.get(plot.x).unwrap().get(plot.y).unwrap() == PlotFlooring::WALL;
    // }
    fn get_closest_boxes_range(&self, centre: f32, size: u8) -> (i32, i32){
        let centre_coord = (centre / ARENA_PLOT_SIZE).floor() as i32;
        if size % 2 == 0{
            if size == 0{
                return (centre_coord, centre_coord);
            }
            let in_shortest_direction = ((size / 2) - 1) as i32;
            if centre % ARENA_PLOT_SIZE > ARENA_PLOT_SIZE / 2.0{
                // Include 1 extra on right.
                return (centre_coord - in_shortest_direction, centre_coord + in_shortest_direction + 1 + 1 /*Top exclusive*/)
            }else{
                // Include 1 extra on left.
                return (centre_coord - in_shortest_direction - 1, centre_coord + in_shortest_direction + 1 /*Top exclusive*/)
            }
        }else{
            // Odd. Easy case. Just do floor(x/2) in each direction.
            let in_each_direction = (size / 2) as i32;
            return (centre_coord - in_each_direction, centre_coord + in_each_direction + 1 /*Top exclusive*/)
        }
    }
    pub fn set_flooring(&mut self, coords: &Plot, floor: PlotFlooring){
        let flooring = self.flooring.get_mut(coords.x).unwrap().get_mut(coords.y).unwrap();
        *flooring = floor;
    }
    pub fn get_flooring(&self, coords: &Plot) -> PlotFlooring {
        return *self.flooring.get(coords.x).unwrap().get(coords.y).unwrap();
    }
    pub fn pathfind(&mut self, start: PointFloat, end: PointFloat) -> Vec<PointFloat>{
        return vec![PointFloat::new(0.0,0.0)];
    }
    pub fn get_plot_boxes(&self, centre: PointFloat, plot_size: PlotSize) -> Option<Vec<Plot>>{
        let mut plots = vec![];
        let (minx, maxx) = self.get_closest_boxes_range(centre.x, plot_size.x);
        let (miny, maxy) = self.get_closest_boxes_range(centre.y, plot_size.y);
        for x in minx..maxx{
            for y in miny..maxy{
                let plot = self.get_ploti(x, y);
                if let Some(plot) = plot{
                    plots.push(plot);
                }else{
                    return None;
                }
            }
        }
        return Some(plots);
    }
    pub fn get_blank_performance_map(pathing: &PathingMap) -> PerformanceMap{
        let width_pixels = ARENA_PLOT_SIZE * pathing.len() as f32;
        let height_pixels = ARENA_PLOT_SIZE * pathing.get(0).unwrap().len() as f32;
        let required_performance_boxes_width = (width_pixels / PERFORMANCE_MAP_BOX_SIZE) as usize + 1;
        let required_performance_boxes_height = (height_pixels / PERFORMANCE_MAP_BOX_SIZE) as usize + 1;
        let mut performance_map = vec![vec![vec![]; required_performance_boxes_height]; required_performance_boxes_width]; // TODO: Real values.
        return performance_map;
    }
    pub fn load(filepath: String) -> Self{
        let mut lock = crate::rts::game::game_resources::GAME_RESOURCES.lock().unwrap();
        let image = lock.get_image(filepath);
        let mut pathing = vec![];
        assert_eq!(image.height(), image.width(), "Not supported! (yet maybe)");
        for x in 0..image.width(){
            pathing.push(vec![]);
            for y in 0..image.height(){
                let (r,g,b,a) = image.get_pixel(x as u32, y as u32).channels4();
                let color = ggez::graphics::Color::from_rgba(r,g,b,a);
                pathing.get_mut(x as usize).unwrap().push(PlotFlooring::from_color(color));
            }
        }
        let performance_map = Self::get_blank_performance_map(&pathing);
        Self{
            flooring: pathing,
            performance_map,
        }
    }

    pub fn get_box_length(&self) -> f32{
        ARENA_PLOT_SIZE
    }
    pub fn get_box_size(&self) -> PointFloat{
        PointFloat::new(ARENA_PLOT_SIZE as f32, ARENA_PLOT_SIZE as f32)
    }
    pub fn get_length(&self) -> f32{
        ARENA_PLOT_SIZE * self.flooring.len() as f32
    }
    pub fn get_size(&self) -> PointFloat{
        PointFloat::new(self.get_length() as f32, self.get_length() as f32)
    }
    pub fn get_top(&self) -> i32{
        0
    }
    pub fn get_bottom(&self) -> i32{
        self.get_length() as i32
    }
    pub fn get_left(&self) -> i32{
        0
    }
    pub fn get_right(&self) -> i32{
        self.get_length() as i32
    }
    pub fn get_top_left(&self) -> PointFloat{
        PointFloat::new(self.get_left() as f32, self.get_top() as f32)
    }
    pub fn get_bottom_right(&self) -> PointFloat{
        PointFloat::new(self.get_right() as f32, self.get_bottom() as f32)
    }
}