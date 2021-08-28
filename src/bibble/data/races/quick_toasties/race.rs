use crate::bibble::data::data_types::*;

pub fn race(data: &mut GameData){
    let mut starting_effects = vec![];

    for _ in 0..1{
        starting_effects.push(EffectToPoint::SPAWN_UNIT(UnitID::BREAD));
    }
    for _ in 0..5{
        starting_effects.push(EffectToPoint::SPAWN_UNIT(UnitID::DOUGH));
    }


    data.races.insert(RaceID::QUICK_TASTERS, RaceMould{
        spawn_effect: EffectToPoint::COMPOSITE(starting_effects),
    });
}