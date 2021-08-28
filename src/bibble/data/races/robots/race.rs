use crate::bibble::data::data_types::*;

pub fn race(data: &mut GameData){
    let mut starting_effects = vec![];

    for _ in 0..1{
        starting_effects.push(EffectToPoint::SPAWN_UNIT(UnitID::CONSTRUCTOR));
    }
    for _ in 0..1{
        starting_effects.push(EffectToPoint::SPAWN_UNIT(UnitID::SCUTTLER));
    }
    for _ in 0..1{
        starting_effects.push(EffectToPoint::SPAWN_UNIT(UnitID::FOUNDRY));
    }


    data.races.insert(RaceID::ROBOTS, RaceMould{
        spawn_effect: EffectToPoint::COMPOSITE(starting_effects),
    });
}