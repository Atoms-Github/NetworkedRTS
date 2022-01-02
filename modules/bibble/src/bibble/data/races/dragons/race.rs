use crate::bibble::data::data_types::*;

pub fn race(data: &mut GameData){
    let mut starting_effects = vec![];

    for _ in 0..1{
        starting_effects.push(EffectToPoint::SPAWN_UNIT(UnitID::VOLCANO));
    }

    data.races.insert(RaceID::DRAGONS, RaceMould{
        spawn_effect: EffectToPoint::COMPOSITE(starting_effects),
        icon: "dragon_icon.png".to_string()
    });
}