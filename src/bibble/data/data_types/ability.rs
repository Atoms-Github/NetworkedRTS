use crate::bibble::data::data_types::*;
use serde::*;
use winit::VirtualKeyCode;

#[repr(u16)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AbilityID{
    WALK,
    NONE,
    ATTACK_GROUND,
    SPAWN_SCUTTLER,
    WEP_GLAIVES,
}
/// How 'bout, we just have a special class of ability, which resolves into appropriate behaviours and effects.
/// Ain't work with cost, but rest fine.
///
/// Hmm. Or maybe better to just use trainer attribute of buildings, since then we can set rally points.
#[derive(Serialize, Deserialize, Clone)]
pub struct AbilityMould{
    pub cost: f32,
    pub targetting: AbilityTargetType,
    pub button_info: ButtonMould,
    pub range: f32,
    pub casting_time: f32,
}
// Let's make buttons stateless.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ButtonMould{
    pub color: (u8, u8, u8),
    pub hotkey: VirtualKeyCode,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum AbilityTargetType{
    NoTarget(EffectToUnit),
    Unit(EffectUnitToUnit),
    Point(EffectUnitToPoint),

    // Can add Points (two points) if I need it.
}

