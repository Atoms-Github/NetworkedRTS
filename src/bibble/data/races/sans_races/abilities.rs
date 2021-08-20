use crate::bibble::data::data_types::*;
use nalgebra::Point2;


pub fn abilities(data: &mut GameData){
    data.abilities.insert(AbilityID::WALK, AbilityMould{
        cost: 0.0,
        targetting: AbilityTargetType::Point(EffectUnitToPoint::NOTHING),
        button_info: ButtonMould { color: (0, 255, 0), hotkey: VirtualKeyCode::M },
        range: 0.0,
        casting_time: 0.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::ATTACK_GROUND, AbilityMould{
        cost: 0.0,
        targetting: AbilityTargetType::Point(EffectUnitToPoint::NOTHING),
        button_info: ButtonMould { color: (255, 130, 100), hotkey: VirtualKeyCode::Minus },
        range: 0.0,
        casting_time: 0.0,
        cooldown: 0.0
    });
}