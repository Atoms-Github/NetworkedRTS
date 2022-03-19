use crate::bibble::data::data_types::*;
use nalgebra::Point2;
use winit::event::VirtualKeyCode;
use crate::pub_types::PointFloat;

pub fn abilities(data: &mut GameData){
    data.abilities.insert(AbilityID::TRAIN_RED_DRAGON_EGG, AbilityMould{
        cost: 200.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::RED_DRAGON_EGG))),
        button_info: ButtonMould{
            color: (255, 100, 100),
            hotkey: VirtualKeyCode::R
        },
        range: 0.0,
        casting_time: 1000.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::TRAIN_SMALL_DRAGON, AbilityMould{
        cost: 20.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::SMALL_DRAGON))),
        button_info: ButtonMould{
            color: (150, 200, 50),
            hotkey: VirtualKeyCode::S
        },
        range: 0.0,
        casting_time: 500.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::BUILD_VOLCANO, AbilityMould{
        cost: 100.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::VOLCANO))),
        button_info: ButtonMould{
            color: (150, 120, 200),
            hotkey: VirtualKeyCode::V
        },
        range: 0.0,
        casting_time: 1000.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::WEP_RED_DRAGON, AbilityMould{
        cost: 0.0,
        targetting: AbilityTargetType::Unit(EffectUnitToUnit::INSTA_AFFECT_TARGET(EffectToUnit::DAMAGE(EffectToUnitDamage{
            amount: 100.0
        }))),
        button_info: ButtonMould{
            color: (255, 0, 0),
            hotkey: VirtualKeyCode::Minus
        },
        range: 300.0,
        casting_time: 0.0,
        cooldown: 500.0
    });
    data.abilities.insert(AbilityID::WEP_SMALL_DRAGON, AbilityMould{
        cost: 0.0,
        targetting: AbilityTargetType::Unit(EffectUnitToUnit::INSTA_AFFECT_TARGET(EffectToUnit::DAMAGE(EffectToUnitDamage{
            amount: 10.0
        }))),
        button_info: ButtonMould{
            color: (255, 0, 0),
            hotkey: VirtualKeyCode::Minus
        },
        range: 100.0,
        casting_time: 0.0,
        cooldown: 500.0
    });
}