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
pub struct AbilitiesComp {
    pub abilities: [AbilityID; 5],
}


pub static WEAPON_SYS: System<ResourcesPtr> = System{
    run,
    name: "weapon"
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){


}