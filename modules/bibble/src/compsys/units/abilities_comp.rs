use crate::*;
use ggez::event::MouseButton;

use ggez::graphics::Rect;
use std::ops::Div;

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
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    // Increment time since use timers.
    for (unit_id, abilities)
    in CompIter1::<AbilitiesComp>::new(c) {
        for ability in &mut abilities.abilities{
            ability.time_since_use += meta.delta;
        }
    }
}










