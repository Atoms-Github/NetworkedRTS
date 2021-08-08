use super::*;
use nalgebra::{Point, Point2};

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaponID {
    GLAIVES,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitID {
    SCUTTLER,
    CONSTRUCTOR,
    FOUNDRY
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActorID {
    DISCIPLE,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProjectileID {
    DISCIPLE_GLAIVE,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RaceID {
    ROBOTS,
}


pub struct UnitMould {
    pub radius: f32,
    pub actor: ActorMould,
    pub weapons: Vec<WeaponID>,
    pub unit_flavour: UnitFlavour,
    pub unit_cost: u32,
}

pub enum UnitFlavour{
    STRUCTURE(StructureFlavourInfo),
    HIKER
}
pub struct StructureFlavourInfo{
    pub footprint: Point2<u8>,
}

pub struct WeaponMould {
    pub effect: EffectUnitToUnit,
    pub cooldown: f32,
    pub range: f32,
}

pub struct ActorMould {
    pub colour: (u8, u8, u8),
}

pub struct ProjectileMould {
    pub actor_id: ActorID,
    pub speed: f32,
}

pub struct RaceMould {
    pub spawn_effect: EffectToPoint,
}
