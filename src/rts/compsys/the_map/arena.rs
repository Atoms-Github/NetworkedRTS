
use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::rts::game::game_state::GameResources;
use ggez::event::MouseButton;

pub const ARENA_DIMENSIONS: usize = 4;
pub const ARENA_SQUARE_SIZE: usize = 200;
pub const ARENA_WIDTH : usize = ARENA_DIMENSIONS * ARENA_SQUARE_SIZE;

pub struct ArenaComp {
    pub pathing: [[bool; ARENA_DIMENSIONS]; ARENA_DIMENSIONS]
}

impl ArenaComp {
    pub fn get_box_length(&self) -> usize{
        ARENA_SQUARE_SIZE
    }
    pub fn get_box_size(&self) -> PointFloat{
        PointFloat::new(ARENA_SQUARE_SIZE as f32, ARENA_SQUARE_SIZE as f32)
    }
    pub fn get_length(&self) -> i32{
        (ARENA_DIMENSIONS * ARENA_SQUARE_SIZE) as i32
    }
    pub fn get_size(&self) -> PointFloat{
        PointFloat::new(self.get_length() as f32, self.get_length() as f32)
    }
    pub fn get_top(&self) -> i32{
        0
    }
    pub fn get_bottom(&self) -> i32{
        self.get_length()
    }
    pub fn get_left(&self) -> i32{
        0
    }
    pub fn get_right(&self) -> i32{
        self.get_length()
    }
    pub fn get_top_left(&self) -> PointFloat{
        PointFloat::new(self.get_left() as f32, self.get_top() as f32)
    }
    pub fn get_bottom_right(&self) -> PointFloat{
        PointFloat::new(self.get_right() as f32, self.get_bottom() as f32)
    }
}