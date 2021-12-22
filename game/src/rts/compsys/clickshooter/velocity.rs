use crate::rts::compsys::jigsaw::jigsaw_game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VelocityComp {
    pub vel: PointFloat,
}

pub static VELOCITY_SYS: System = System{
    run,
    name: "velocity"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    for (entity_id, velocity, position) in CompIter2::<VelocityComp, PositionComp>::new(c){
        position.pos += &velocity.vel;
    }
}