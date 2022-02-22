use crate::*;

use ggez::event::MouseButton;
use ggez::graphics::Color;

pub const ARENA_PLOT_SIZE: f32 = 50.0;
pub const PERFORMANCE_MAP_BOX_SIZE: f32 = 100.0;


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
    pub flooring: ScaledGrid<PlotFlooring>,
    pub pathing: ScaledGrid<bool>,
    pub performance_map: ScaledGrid<Vec<GlobalEntityID>>,
    pub pathgrinder: PathGrinder,
}


impl ArenaComp {
    pub fn clear_performance_map(&mut self){
        for column in self.performance_map.grid.raw_mut(){
            for square in column{
                square.clear();
            }
        }
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
                if box_x >= 0 && box_x < self.performance_map.grid.len_x() as i32{
                    let column = self.performance_map.grid.raw().get(box_x as usize).unwrap();
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
        self.performance_map.get_by_point_mut(position).unwrap().push(entity);
    }
    pub fn is_point_walkable(&self, loc: &PointFloat) -> bool{
        return *self.pathing.get_by_point(loc).unwrap_or(&false);
    }
    pub fn is_box_walkable(&self, loc: &GridBox) -> bool{
        return *self.pathing.get(loc).unwrap_or(&false);
    }
    pub fn get_plot(&self, position: &PointFloat) -> Option<GridBox>{
        return self.flooring.get_grid_coord_maybe(position);
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
        // TODO: There's probably a 1 liner much easier way of doing this.
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
    pub fn set_flooring(&mut self, coords: &GridBox, floor: PlotFlooring){
        self.pathing.grid.set(&coords, floor.can_walk_over());
        self.flooring.grid.set(&coords, floor);
    }
    pub fn get_flooring(&self, coords: &GridBox) -> PlotFlooring {
        return self.flooring.get_unwrap(coords).clone();
    }
    pub fn pathfind(&mut self, start: PointFloat, end: PointFloat, unit_berth: f32) -> Vec<PointFloat>{
        let results = self.pathgrinder.pathfind(&self.pathing, start, end, unit_berth);
        return results;
    }
    pub fn get_plot_boxes(&self, centre: PointFloat, plot_size: PlotSize) -> Option<Vec<GridBox>>{
        let mut boxes = vec![];
        let (minx, maxx) = self.get_closest_boxes_range(centre.x, plot_size.x);
        let (miny, maxy) = self.get_closest_boxes_range(centre.y, plot_size.y);

        let top_left = GridBox::new(minx, miny);
        let bottom_right = GridBox::new(maxx, maxy);
        if self.flooring.grid.is_valid(&top_left) && self.flooring.grid.is_valid(&bottom_right){
            for (gridbox, flooring) in
            self.flooring.grid.iter_square(top_left, bottom_right){
                boxes.push(gridbox);
            }
        }else{
            return None;
        }
        return Some(boxes);
    }
    pub fn new() -> Self{
        Self{
            flooring: ScaledGrid::new(ARENA_PLOT_SIZE),
            pathing: ScaledGrid::new(ARENA_PLOT_SIZE),
            performance_map: ScaledGrid::new( PERFORMANCE_MAP_BOX_SIZE),
            pathgrinder: PathGrinder {}
        }
    }
    pub fn load_map(&mut self, filepath: String){
        let mut lock = bibble::::game_resources::GAME_RESOURCES.lock().unwrap();
        let image = lock.get_image(filepath);

        let bottom_right_corner = GridBox::new(image.width() as i32 - 1, image.height() as i32 - 1);
        self.flooring.grid.resize_to_fit(&bottom_right_corner);
        self.pathing.grid.resize_to_fit(&bottom_right_corner);
        for x in 0..image.width(){
            for y in 0..image.height(){
                let (r,g,b,a) = image.get_pixel(x as u32, y as u32).channels4();
                let color = ggez::graphics::Color::from_rgba(r,g,b,a);
                let flooring = PlotFlooring::from_color(color);
                self.set_flooring(&GridBox::new(x as i32, y as i32), flooring);
            }
        }
        self.performance_map.grid.resize_to_fit(&GridBox::new(
            (self.pathing.get_scaled_width() / PERFORMANCE_MAP_BOX_SIZE) as i32,
            (self.pathing.get_scaled_height() / PERFORMANCE_MAP_BOX_SIZE) as i32
        ));
    }

    pub fn get_box_length(&self) -> f32{
        ARENA_PLOT_SIZE
    }
    pub fn get_box_size(&self) -> PointFloat{
        PointFloat::new(ARENA_PLOT_SIZE as f32, ARENA_PLOT_SIZE as f32)
    }
    pub fn get_length(&self) -> f32{
        return self.flooring.get_scaled_width();
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
    pub fn get_centre(&self) -> PointFloat{
        PointFloat::new((self.get_left()  + self.get_right()) as f32 / 2.0, (self.get_top()  + self.get_bottom()) as f32 / 2.0,)
    }
    pub fn get_bottom_right(&self) -> PointFloat{
        PointFloat::new(self.get_right() as f32, self.get_bottom() as f32)
    }
}
