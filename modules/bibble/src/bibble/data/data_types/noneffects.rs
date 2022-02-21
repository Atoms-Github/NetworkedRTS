use serde::*;
use game::pub_types::PointFloat;
use crate::bibble::data::data_types::*;
use crate::bibble::*;
use nalgebra::Point2;

#[repr(u16)]
#[derive(Serialize, Deserialize ,Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaponID {
    GLAIVES,
}

#[repr(u16)]
#[derive(Serialize, Deserialize ,Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitID {
    ROBO_SPIDER,
    ROBO_LOBBER,
    CONSTRUCTOR,
    FACTORY,
    OIL_WELL,

    DOUGH,
    BREAD,
    DOUGH_LAUNCHER,

    RED_DRAGON,
    RED_DRAGON_EGG,
    SMALL_DRAGON,
    VOLCANO,
}

#[repr(u16)]
#[derive(Serialize, Deserialize ,Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActorID {
    DISCIPLE,
}



#[repr(u16)]
#[derive(Serialize, Deserialize ,Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RaceID {
    ROBOTS,
    QUICK_TASTERS,
    DRAGONS,
    DWARVES,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UnitMould {
    pub radius: f32,
    pub actor: ActorMould,
    pub weapons: Vec<AbilityID>,
    pub abilities: Vec<AbilityID>,
    pub unit_flavour: UnitFlavour,
    pub periodic_gain: ResourceBlock,
    pub life: f32,
}
impl UnitMould{
    pub fn add_weapon(&mut self, data: &mut GameData, id: AbilityID, effect: EffectUnitToUnit, range: f32, cooldown: f32){
        self.weapons.push(id);
        self.abilities.push(id);
        data.abilities.insert(id, AbilityMould{
            cost: 0.0,
            targetting: AbilityTargetType::SingleTarget(AbilitySingleTarget{
                target: AbilitySingleTargetType::Unit(effect),
                graphic: AbilitySingleTargetGraphic::NOTHING
            }),
            button_info: ButtonMould{
                color: (255, 0, 0),
                hotkey: VirtualKeyCode::Minus
            },
            range,
            casting_time: 0.0,
            cooldown,
        });
    }
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum UnitFlavour{
    STRUCTURE(StructureFlavourInfo),
    HIKER(HikerFlavourInfo)
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct HikerFlavourInfo{
    pub movespeed: f32,
    pub fly: bool,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct StructureFlavourInfo{
    pub footprint: Point2<u8>,
    pub required_under_material: PlotFlooring,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct WeaponMould {
    pub effect: EffectUnitToUnit,
    pub cooldown: f32,
    pub range: f32,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ActorMould {
    pub image: String,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RaceMould {
    pub spawn_effect: EffectToPoint,
    pub icon: String,
}
