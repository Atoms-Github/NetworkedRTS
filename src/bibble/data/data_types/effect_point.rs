use crate::bibble::data::data_types::UnitID;
use serde::*;

#[repr(u16)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EffectToPoint{
    DETONATE,
    SPAWN_UNIT(UnitID),
    COMPOSITE(Vec<EffectToPoint>)
}