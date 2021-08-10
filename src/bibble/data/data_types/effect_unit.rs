use super::*;

pub enum EffectToUnit{
    DAMAGE(EffectToUnitDamage),
    ADD_WEAPON(EffectToUnitAddWeapon),
    EFFECT_TO_POINT(EffectToPoint),
    ISSUE_ORDER,
    MORPH
}

pub struct EffectToUnitDamage{
    pub amount: f32,
}
pub struct EffectToUnitStartTrain{
    pub unit: UnitID
}

pub struct EffectToUnitAddWeapon{
    pub weapon_id: WeaponID,
}
