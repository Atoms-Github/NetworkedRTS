use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use crate::rts::game::game_state::{ARENA_ENT_ID, GameResources};
use ggez::graphics::Rect;
use std::ops::Div;

use crate::bibble::data::data_types::{WeaponID, AbilityID};
use crate::bibble::effect_resolver::revolver::Revolver;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AbilitiesComp {
    pub abilities: [AbilityID; 5],

}


pub static WEAPON_SYS: System<ResourcesPtr> = System{
    run,
    name: "weapon"
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    let mut revolver = Revolver::new(c, &res.game_data);

    for (shooter_id, weapon, owned_shooter, position_shooter) in CompIter3::<WeaponComp, OwnedComp, PositionComp>::new(c) {
        weapon.time_since_shot += 1.0;
        let weapon_mould = res.game_data.get_weapon(weapon.weapon_id);
        if weapon_mould.cooldown < weapon.time_since_shot{
            for (target_id, owned_target, position_target, life_target) in CompIter3::<OwnedComp, PositionComp, LifeComp>::new(c) {
                let in_range = (position_target.pos.clone() - &position_shooter.pos).magnitude_squared() < weapon_mould.range.powf(2.0);
                if shooter_id != target_id && owned_target.owner != owned_shooter.owner && in_range{
                    weapon.time_since_shot = 0.0;
                    life_target.life -= 10.0;
                    break;
                }
            }
        }
    }
    revolver.end().move_into(ent_changes);

}