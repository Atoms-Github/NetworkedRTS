use crate::bibble::data::data_types::*;
use nalgebra::Point2;


pub fn abilities(data: &mut GameData){
    data.abilities.insert(AbilityID::WALK, AbilityMould{
        cost: 0.0,
        targetting: AbilityTargetType::Point(EffectUnitToPoint::TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::SCUTTLER))),
        button_info: ButtonMould { color: (0, 255, 0), hotkey: VirtualKeyCode::M },
        range: 0.0,
        casting_time: 0.0,
    });
}