use super::*;

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaponID{
    GLAIVES,
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitID{
    DISCIPLE,
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActorID{
    DISCIPLE,
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProjectileID{
    DISCIPLE_GLAIVE,
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RaceID{
    DISCIPLE_GLAIVE,
}


pub struct UnitMould{
}
pub struct WeaponMould{
    pub effect: EffectUnitToUnit,
    pub cooldown: f32,
}
pub struct ActorMould{
    pub image: String
}
pub struct ProjectileMould{
    pub actor_id: ActorID,
    pub speed: f32,
}
pub struct RaceMould{
    pub spawn_effect: EffectToPoint
}
