use super::*;

pub enum EffectToUnit{
    DAMAGE(EffectToUnitDamage),
    ADD_WEAPON(EffectToUnitAddWeapon),
    ISSUE_ORDER,
    MORPH
}

pub struct EffectToUnitDamage{
    pub amount: f32,
}

pub struct EffectToUnitAddWeapon{
    pub weapon_id: WeaponID,
}
