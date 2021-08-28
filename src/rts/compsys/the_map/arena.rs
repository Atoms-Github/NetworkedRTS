
use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::rts::game::game_state::RenderResources;
use ggez::event::MouseButton;
use image::Pixel;
use mopa::Any;

pub const ARENA_SQUARE_SIZE: usize = 50;
pub const PERFORMANCE_MAP_BOX_SIZE: f32 = 100.0;
pub type PathingMap = Vec<Vec<bool>>;
pub type PerformanceMap = Vec<Vec<Vec<GlobalEntityID>>>;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ArenaComp {
    pub pathing: PathingMap,
    pub performance_map: PerformanceMap,
}


impl ArenaComp {
    pub fn clear_performance_map(&mut self){
        self.performance_map = Self::get_blank_performance_map(&self.pathing);
    }
    pub fn get_nearby_performance_map_entities(&self, position: &PointFloat, radius: f32) -> Vec<GlobalEntityID>{
        // Steps:
        // Get the radius.
        // Round it up to the next box.
        // That's how many out you want to go, not including yourself, in each direction.
        let mut collected_ids = vec![];
        let out_in_each_direction = (radius / PERFORMANCE_MAP_BOX_SIZE) as i32 + 1;
        for x_offset in -out_in_each_direction..out_in_each_direction + 1{
            for y_offset in -out_in_each_direction..out_in_each_direction + 1{
                let box_x = (position.x / PERFORMANCE_MAP_BOX_SIZE) as i32 + x_offset;
                let box_y = (position.y / PERFORMANCE_MAP_BOX_SIZE) as i32 + y_offset;
                if box_x >= 0 && box_x < self.performance_map.len() as i32{
                    let column = self.performance_map.get(box_x as usize).unwrap();
                    if box_y >= 0 && box_y < column.len() as i32{
                        let mut square = column.get(box_y as usize).unwrap().clone();
                        collected_ids.append(&mut square);
                    }
                }
            }
        }
        return collected_ids;

    }
    pub fn register_performance_map_entity(&mut self, entity: GlobalEntityID, position: &PointFloat){
        let x = (position.x / PERFORMANCE_MAP_BOX_SIZE) as usize;
        let y = (position.y / PERFORMANCE_MAP_BOX_SIZE) as usize;
        self.performance_map.get_mut(x).unwrap().get_mut(y).unwrap().push(entity);
    }
    pub fn get_blank_performance_map(pathing: &PathingMap) -> PerformanceMap{
        let width_pixels = ARENA_SQUARE_SIZE * pathing.len();
        let height_pixels = ARENA_SQUARE_SIZE * pathing.get(0).unwrap().len();
        let required_performance_boxes_width = (width_pixels as f32 / PERFORMANCE_MAP_BOX_SIZE) as usize + 1;
        let required_performance_boxes_height = (height_pixels as f32 / PERFORMANCE_MAP_BOX_SIZE) as usize + 1;
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
                pathing.get_mut(x as usize).unwrap().push(r == 255);
            }
        }
        let performance_map = Self::get_blank_performance_map(&pathing);
        Self{
            pathing,
            performance_map,
        }
    }

    pub fn get_box_length(&self) -> usize{
        ARENA_SQUARE_SIZE
    }
    pub fn get_box_size(&self) -> PointFloat{
        PointFloat::new(ARENA_SQUARE_SIZE as f32, ARENA_SQUARE_SIZE as f32)
    }
    pub fn get_length(&self) -> usize{
        ARENA_SQUARE_SIZE * self.pathing.len()
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