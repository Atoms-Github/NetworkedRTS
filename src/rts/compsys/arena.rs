
use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::rts::game::game_state::GameResources;
use ggez::event::MouseButton;

pub const ARENA_SIZE: usize = 16;
pub struct ArenaComp {
    pub pathing: [[bool; ARENA_SIZE]; ARENA_SIZE]
}

impl ArenaComp {

}