use crate::bibble::data::data_types::{EffectUnitToPoint, EffectToUnit, EffectUnitToUnit};

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AbilityID{
    WALK
}

pub struct AbilityMould{
    cost: f32,
    targetting: AbilityTargetType,
    button_info: ButtonMould,
    range: f32
}
// Let's make buttons stateless.
pub struct ButtonMould{
    color: (u8, u8, u8),

}
pub enum AbilityTargetType{
    NoTarget(EffectToUnit),
    Unit(EffectUnitToUnit),
    Point(EffectUnitToPoint),
    // Can add Points (two points) if I need it.
}

