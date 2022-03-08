use crate::*;
use crate::bibble::data::data_types::{GameData, RaceID, RaceMould, EffectToPoint};
use crate::bibble::data::data_types::*;

mod units;
mod weapons;
mod abilities;


pub fn gather(data: &mut GameData){
    data.abilities.insert(AbilityID::WALK, AbilityMould{
        cost: 0.0,
        targetting: AbilityTargetType::SingleTarget(AbilitySingleTarget{
            target: AbilitySingleTargetType::Point(EffectUnitToPoint::NOTHING),
            graphic: AbilitySingleTargetGraphic::NOTHING
        }),
        button_info: ButtonMould { color: (0, 255, 0), hotkey: VirtualKeyCode::M },
        range: 0.0,
        casting_time: 0.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::ATTACK_GROUND, AbilityMould{
        cost: 0.0,
        targetting: AbilityTargetType::SingleTarget(AbilitySingleTarget{
            target: AbilitySingleTargetType::Point(EffectUnitToPoint::NOTHING),
            graphic: AbilitySingleTargetGraphic::NOTHING
        }),
        button_info: ButtonMould { color: (255, 130, 0), hotkey: VirtualKeyCode::Minus },
        range: 0.0,
        casting_time: 0.0,
        cooldown: 0.0
    });

    weapons::weapons(data);
    units::units(data);
    abilities::abilities(data);
}