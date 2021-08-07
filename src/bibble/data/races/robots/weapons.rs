use crate::bibble::data::data_types::*;

pub fn weapons(data: &mut GameData){
    data.weapons.insert(WeaponID::GLAIVES,
    WeaponMould{
        effect: EffectUnitToUnit::INSTA_AFFECT_TARGET(EffectToUnit::DAMAGE(EffectToUnitDamage{
            amount: 10.0
        })),
        cooldown: 60.0
    });
}