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
    for (shooter_id, weapon, owned, position) in CompIter3::<WeaponComp, OwnedComp, PositionComp>::new(c) {
        weapon.time_since_shot += 1.0;
        
        for (target_id, owned, position) in CompIter2::<OwnedComp, PositionComp>::new(c) {
            if shooter_id != target_id{

            }
        }
    }

}