use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use ggez::graphics::Rect;
use std::ops::Div;
use std::ops::Mul;

use crate::bibble::data::data_types::{WeaponID, AbilityID, EffectToUnit};
use crate::bibble::effect_resolver::revolver::Revolver;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SeekingProjComp {
    pub speed: f32,
    pub hit_effect: EffectToUnit,
    pub target: GlobalEntityID,
}

pub static SEEKING_PROJECTILES_COMP: System = System{
    run,
    name: "seeking_projectiles"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    let mut revolver = Revolver::new(c);
    for (proj_id, seeking_proj, position, owner) in
    CompIter3::<SeekingProjComp, PositionComp, OwnedComp>::new(c){
        if c.ent_alive(seeking_proj.target){
            let target_loc = &c.get_unwrap::<PositionComp>(seeking_proj.target).pos;
            let a = target_loc;
            let b = &position.pos;
            let move_dist = seeking_proj.speed * crate::netcode::common::time::timekeeping::FRAME_DURATION_MILLIS;
            // if nalgebra::distance_squared(&a.clone(), &b.clone()) <= seeking_proj.speed.powf(2.0){
            if (a.clone() - b).magnitude_squared() <= move_dist.powf(2.0){
                revolver.revolve_to_unit(proj_id.get_owner_tech_tree(c), &seeking_proj.hit_effect, seeking_proj.target);
                // Hit target. Kms.
                ent_changes.deleted_entities.push(proj_id);
            }else{ // Move towards. Out of range.
                let target_loc = &c.get_unwrap::<PositionComp>(seeking_proj.target).pos;
                position.pos += (target_loc.clone() - &position.pos).normalize().mul(move_dist);
            }
        }else{
            // Target dead. Kms.
            ent_changes.deleted_entities.push(proj_id);
        }
    }
    revolver.end().apply(c);
}










