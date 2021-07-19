
use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::MouseButton;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
use std::ops::Mul;

pub struct HikerComp { // Walks comp, but includes fliers and sailers too.
    pub destination: Option<PointFloat>, // TODO: Change to in_mem_vec.
    pub speed: f32,
    pub quest_importance: u8,
}


pub static HIKER_SYS: System<ResourcesPtr> = System{
    run
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (unit_id, hiker, position) in CompIter2::<HikerComp, PositionComp>::new(c) {
        // Moving the units.
        let mut made_destination = false;
        if let Some(my_destination) = &mut hiker.destination{
            if (my_destination.clone() - &position.pos).magnitude() < hiker.speed{
                position.pos = my_destination.clone();
                made_destination = true;
            }else{
                position.pos += (my_destination.clone() - &position.pos).normalize().mul(hiker.speed);

            }
        }
        if made_destination{
            hiker.destination = None;
        }

    }
}

