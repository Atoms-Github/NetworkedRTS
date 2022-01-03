use bibble::::data::data_types::{UnitID, EffectToUnit};
use serde::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EffectToPoint{
    DETONATE,
    SPAWN_UNIT(UnitID),
    BUILD_BUILDING(UnitID),
    COMPOSITE(Vec<EffectToPoint>),
    EFFECT_NEARBY_UNITS(Box<EffectToUnit>, f32),
}