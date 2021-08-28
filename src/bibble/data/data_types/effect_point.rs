use crate::bibble::data::data_types::{UnitID, EffectToUnit};
use serde::*;

#[repr(u16)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EffectToPoint{
    DETONATE,
    SPAWN_UNIT(UnitID),
    COMPOSITE(Vec<EffectToPoint>),
    EFFECT_NEARBY_UNITS(Box<EffectToUnit>, f32)
}