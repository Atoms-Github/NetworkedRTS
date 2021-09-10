use super::*;
use nalgebra::{Point, Point2};
use serde::*;
use crate::pub_types::PointFloat;
use crate::rts::compsys::ResourceBlock;

#[repr(u16)]
#[derive(Serialize, Deserialize ,Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaponID {
    GLAIVES,
}

#[repr(u16)]
#[derive(Serialize, Deserialize ,Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitID {
    SCUTTLER,
    CONSTRUCTOR,
    FOUNDRY,

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
