use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use crate::rts::game::game_state::{ARENA_ENT_ID, RenderResources};
use ggez::graphics::Rect;
use std::ops::Div;

use crate::bibble::data::data_types::{WeaponID, AbilityID};
use crate::bibble::effect_resolver::revolver::Revolver;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct WeaponComp {
    pub time_since_shot: f32,
    pub wep_ability_id: AbilityID,
}

pub static WEAPON_SYS: System<ResourcesPtr> = System{
    run,
    name: "weapon"
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    let mut revolver = Revolver::new(c);
    // Increment time since shot.
    for (shooter_id, weapon, owned_shooter, position_shooter, orders)
    in CompIter4::<WeaponComp, OwnedComp, PositionComp, OrdersComp>::new(c) {
        weapon.time_since_shot += crate::netcode::common::time::timekeeping::FRAME_DURATION_MILLIS;
    }
    // Check for queuing up 'shoot once' commands.
    for (shooter_id, weapon, owned_shooter, position_shooter, orders)
    in CompIter4::<WeaponComp, OwnedComp, PositionComp, OrdersComp>::new(c) {
        let executing_order = orders.get_executing_order();
        if executing_order.is_none() || executing_order.unwrap().ability == AbilityID::ATTACK_GROUND {
            let data = shooter_id.get_owner_tech_tree(c);
            let ability_mould = data.get_ability(weapon.wep_ability_id);
            // Don't check cooldowns here. That's done on ability execution.
            for (target_id, owned_target, position_target, life_target) in
            CompIter3::<OwnedComp, PositionComp, LifeComp>::new(c) {
                let in_range = (position_target.pos.clone() - &position_shooter.pos).magnitude_squared() < ability_mould.range.powf(2.0);
                if owned_target.owner != owned_shooter.owner && in_range{
                    orders.enqueue(OrderInstance{
                        ability: weapon.wep_ability_id,
                        target: AbilityTargetInstance::UNIT(target_id)
                    }, true);
                    break;
                }
            }
        }
    }
    revolver.end().move_into(ent_changes);
}