use crate::rts::compsys::jigsaw::jigsaw_game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VelocityComp {
    pub vel: PointFloat,
}

pub fn velocity_sys<C>() -> System<C>{
    System{
        run,
        name: "velocity"
    }
}
fn run<C>(c: &mut CompStorage<C>, ent_changes: &mut EntStructureChanges<C>, meta: &SimMetadata){
    for (entity_id, velocity, position) in CompIter2::<VelocityComp, PositionComp>::new(c){
        position.pos += &velocity.vel;
    }
}