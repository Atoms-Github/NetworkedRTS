
use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::event::MouseButton;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
use std::ops::Mul;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LifeComp{
    pub life: f32,
    pub max_life: f32,
}

pub static LIFE_SYS: System = System{
    run,
    name: "life"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    for (unit_id, life) in CompIter1::<LifeComp>::new(c) {
        if life.life <= 0.0{
            ent_changes.deleted_entities.push(unit_id);
        }

    }
}


