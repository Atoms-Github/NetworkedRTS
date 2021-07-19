
use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::MouseButton;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
use std::ops::Mul;

pub struct HikerCollisionComp {
    pub radius: f32
}
pub static HIKER_COLLISION_SYS: System<ResourcesPtr> = System{
    run
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (unit_id_1, hiker_collision_1, hiker_comp_1, position_1) in CompIter3::<HikerCollisionComp, HikerComp, PositionComp>::new(c) {
        for (unit_id_2, hiker_collision_2, hiker_comp_2, position_2) in CompIter3::<HikerCollisionComp, HikerComp, PositionComp>::new(c) {
            if unit_id_1 != unit_id_2{
                let actual_distance_squared =  (position_1.pos.x - position_2.pos.x).powi(2) + (position_1.pos.y - position_2.pos.y).powi(2);
                let min_distance = hiker_collision_1.radius + hiker_collision_2.radius;
                if actual_distance_squared < min_distance.powi(2) {
                    let distance_too_close = min_distance - actual_distance_squared.sqrt();
                    const IMPORTANTER_ONE_BOP_FRACTION : f32 = 0.25;
                    let bop_fraction_for_1 = {
                        if hiker_comp_1.quest_importance == hiker_comp_2.quest_importance{
                            0.5
                        }else if hiker_comp_1.quest_importance > hiker_comp_2.quest_importance{
                            IMPORTANTER_ONE_BOP_FRACTION
                        }else{
                            1.0 - IMPORTANTER_ONE_BOP_FRACTION
                        }
                    };
                    let bop_dist_1 = bop_fraction_for_1 * distance_too_close;
                    let bop_dist_2 = (1.0 - bop_fraction_for_1) * distance_too_close;
                    apply_bop(bop_dist_1, position_1, position_2);
                    apply_bop(bop_dist_2, position_2, position_1);
                }
            }
        }
    }
}

fn apply_bop(bop_dist: f32, boppee: &mut PositionComp, bopper: &PositionComp){
    let pos_diff = boppee.pos.clone() - &bopper.pos;
    let safe_diff = {
        if pos_diff.magnitude_squared() > 0.01{
            pos_diff
        }else{
            PointFloat::new(1.0,0.0)
        }
    };
    let move_dist = safe_diff.normalize().mul(bop_dist);
    boppee.pos += move_dist;
}























