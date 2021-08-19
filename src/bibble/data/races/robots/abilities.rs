use crate::bibble::data::data_types::*;
use nalgebra::Point2;
use winit::VirtualKeyCode;

pub fn abilities(data: &mut GameData){
    data.abilities.insert(AbilityID::SPAWN_SCUTTLER, AbilityMould{
        cost: 50.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::SCUTTLER))),
        button_info: ButtonMould{
            color: (50, 20, 100),
            hotkey: VirtualKeyCode::Q
        },
        range: 0.0,
        casting_time: 200.0,
    });

}