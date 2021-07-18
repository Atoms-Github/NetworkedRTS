use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;

pub struct VelocityComp {
    pub vel: PointFloat,
}

pub static VELOCITY_SYS: System<ResourcesPtr> = System{
    run
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (entity_id, velocity, position) in CompIter2::<VelocityComp, PositionComp>::new(c){
        position.pos += &velocity.vel;
    }
}