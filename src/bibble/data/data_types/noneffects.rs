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
    QUICK_TASTERS
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UnitMould {
    pub radius: f32,
    pub actor: ActorMould,
    pub weapons: Vec<AbilityID>,
    pub abilities: Vec<AbilityID>,
    pub unit_flavour: UnitFlavour,
    pub move_speed: f32,
    pub periodic_gain: ResourceBlock,
    pub life: f32,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum UnitFlavour{
    STRUCTURE(StructureFlavourInfo),
    HIKER
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
    pub size: PointFloat,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RaceMould {
    pub spawn_effect: EffectToPoint,
}
