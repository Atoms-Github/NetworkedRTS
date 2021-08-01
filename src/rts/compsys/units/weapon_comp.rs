use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use crate::rts::game::game_state::{ARENA_ENT_ID, GameResources};
use ggez::graphics::Rect;
use std::ops::Div;

use crate::bibble::data::data_types::WeaponID;


pub struct WeaponComp {
    pub weapon_id: WeaponID,
    pub time_since_shot: f32,

}


pub static WEAPON_SYS: System<ResourcesPtr> = System{
    run
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (shooter_id, weapon, owned_shooter, position_shooter) in CompIter3::<WeaponComp, OwnedComp, PositionComp>::new(c) {
        weapon.time_since_shot += 1.0;

        if res.game_data.get_weapon(weapon.weapon_id).cooldown < weapon.time_since_shot{
            for (target_id, owned_target, position_target, life_target) in CompIter3::<OwnedComp, PositionComp, LifeComp>::new(c) {
                let in_range = (position_target.pos.clone() - &position_shooter.pos).magnitude() < 100.0;
                if shooter_id != target_id && owned_target.owner != owned_shooter.owner && in_range{
                    weapon.time_since_shot = 0.0;
                    life_target.life -= 10.0;
                    break;
                }
            }
        }


    }

}