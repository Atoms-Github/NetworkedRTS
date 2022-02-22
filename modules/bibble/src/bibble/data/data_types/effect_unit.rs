use crate::*;
use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EffectToUnit{
    DAMAGE(EffectToUnitDamage),
    ADD_WEAPON(EffectToUnitAddWeapon),
    EFFECT_TO_POINT(EffectToPoint),
    ISSUE_ORDER,
    MORPH
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EffectToUnitDamage{
    pub amount: f32,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EffectToUnitStartTrain{
    pub unit: UnitID
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EffectToUnitAddWeapon{
    pub weapon_id: WeaponID,
}
