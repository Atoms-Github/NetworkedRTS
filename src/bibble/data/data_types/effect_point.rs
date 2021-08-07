use crate::bibble::data::data_types::UnitID;

#[repr(u16)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EffectToPoint{
    DETONATE,
    SPAWN_UNIT(UnitID),
    COMPOSITE(Vec<EffectToPoint>)
}