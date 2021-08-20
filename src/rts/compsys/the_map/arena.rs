
use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::rts::game::game_state::RenderResources;
use ggez::event::MouseButton;
use image::Pixel;

pub const ARENA_SQUARE_SIZE: usize = 50;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ArenaComp {
    pub pathing: Vec<Vec<bool>>,
}

impl ArenaComp {
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
        Self{
            pathing
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