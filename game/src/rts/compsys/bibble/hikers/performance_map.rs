
use crate::rts::compsys::jigsaw::jigsaw_game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::event::MouseButton;
use winit::event::VirtualKeyCode;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
use std::ops::Mul;
use crate::bibble::data::data_types::AbilityID;
use mopa::Any;
use nalgebra::{distance, distance_squared};
use crate::bibble::effect_resolver::revolver::Revolver;


pub fn performance_map_sys<C>() -> System<C>{
    System{
        run,
        name: "performance_map"
    }
}
fn run<C>(c: &mut CompStorage<C>, ent_changes: &mut EntStructureChanges<C>, meta: &SimMetadata){
    if let Some(arena) = c.find_arena(){
        arena.clear_performance_map();
        for (unit_id, position, owned, life)
        in CompIter3::<PositionComp, OwnedComp, LifeComp>::new(c) {
            arena.register_performance_map_entity(unit_id, &position.pos)
        }
    }
}












