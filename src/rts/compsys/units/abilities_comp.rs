use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use ggez::graphics::Rect;
use std::ops::Div;

use crate::bibble::data::data_types::{WeaponID, AbilityID};
use crate::bibble::effect_resolver::revolver::Revolver;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AbilitiesComp {
    pub abilities: Vec<AbilityInstance>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AbilityInstance{
    pub id: AbilityID,
    pub time_since_use: f32,
}
impl AbilitiesComp{
    pub fn has_ability(&self, id: AbilityID) -> bool{
        for instance in &self.abilities{
            if instance.id == id{
                return true;
            }
        }
        return false;
    }
    pub fn get_ability(&self, id: AbilityID) -> &AbilityInstance{
        for instance in &self.abilities{
            if instance.id == id{
                return instance;
            }
        }
        panic!("Can't find ability {:?}", id);
    }
    pub fn get_ability_mut(&mut self, id: AbilityID) -> &mut AbilityInstance{
        for instance in &mut self.abilities{
            if instance.id == id{
                return instance;
            }
        }
        panic!("Can't find ability {:?}", id);
    }
}


pub static ABILITIES_SYS: System = System{
    run,
    name: "abilities"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    // Increment time since use timers.
    for (unit_id, abilities)
    in CompIter1::<AbilitiesComp>::new(c) {
        for ability in &mut abilities.abilities{
            ability.time_since_use += crate::netcode::common::time::timekeeping::FRAME_DURATION_MILLIS;
        }
    }
}










