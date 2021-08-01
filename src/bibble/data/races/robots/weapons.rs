use crate::bibble::data::data_types::*;

pub fn weapons(data: &mut GameData){
    data.add_weapon(WeaponID::GLAIVES,
    WeaponMould{
        effect: EffectUnitToUnit::INSTA_DAMAGE_TEST,
        cooldown: 60.0
    });
}