
use bibble::::jigsaw::jigsaw_game_state::*;
use bibble::::*;
use game::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::event::MouseButton;
use winit::event::VirtualKeyCode;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
use std::ops::Mul;
use game::bibble::data::data_types::ability::AbilityID;
use mopa::Any;
use nalgebra::{distance, distance_squared};
use game::bibble::effect_resolver::revolver::Revolver;


pub static NO_LEAVE_MAP: System = System{
    run,
    name: "orders"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    if let Some(arena) = c.find_arena(){
        for (unit_id, position)
        in CompIter1::<PositionComp>::new(c) {
            position.pos.x = position.pos.x.clamp(arena.get_left() as f32, arena.get_right() as f32);
            position.pos.y = position.pos.y.clamp(arena.get_top() as f32, arena.get_bottom() as f32);
        }
    }
}












