use crate::bibble::data::data_types::*;
use nalgebra::Point2;
use winit::VirtualKeyCode;

pub fn abilities(data: &mut GameData){
    data.abilities.insert(AbilityID::SPAWN_SCUTTLER, AbilityMould{
        cost: 50.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::SCUTTLER))),
        button_info: ButtonMould{
            color: (150, 120, 200),
            hotkey: VirtualKeyCode::Q
        },
        range: 1000.0,
        casting_time: 1000.0,
    });
    // data.abilities.insert(AbilityID::WEP_GLAIVES, AbilityMould{
    //     cost: 0.0,
    //     targetting: AbilityTargetType::Unit(EffectUnitToUnit::LAUNCH_PROJECTILE()),
    //     button_info: ButtonMould{
    //         color: (150, 120, 200),
    //         hotkey: VirtualKeyCode::Q
    //     },
    //     range: 1000.0,
    //     casting_time: 1000.0,
    // });

}