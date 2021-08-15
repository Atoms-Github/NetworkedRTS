use super::*;
use nalgebra::{Point, Point2};
use serde::*;

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
    FOUNDRY
}

#[repr(u16)]
#[derive(Serialize, Deserialize ,Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActorID {
    DISCIPLE,
}


#[repr(u16)]
#[derive(Serialize, Deserialize ,Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProjectileID {
    DISCIPLE_GLAIVE,
}

#[repr(u16)]
#[derive(Serialize, Deserialize ,Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RaceID {
    ROBOTS,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UnitMould {
    pub radius: f32,
    pub actor: ActorMould,
    pub weapons: Vec<WeaponID>,
    pub abilities: Vec<AbilityID>,
    pub unit_flavour: UnitFlavour,
    pub unit_cost: u32,
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
    pub colour: (u8, u8, u8),
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProjectileMould {
    pub actor_id: ActorID,
    pub speed: f32,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RaceMould {
    pub spawn_effect: EffectToPoint,
}
