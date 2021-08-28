use crate::bibble::data::data_types::*;
use nalgebra::Point2;
use winit::VirtualKeyCode;
use crate::pub_types::PointFloat;

pub fn abilities(data: &mut GameData){
    data.abilities.insert(AbilityID::BAKE_DOUGH, AbilityMould{
        cost: 15.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::DOUGH))),
        button_info: ButtonMould{
            color: (150, 120, 200),
            hotkey: VirtualKeyCode::D
        },
        range: 0.0,
        casting_time: 100.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::BAKE_BREAD, AbilityMould{
        cost: 50.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::BREAD))),
        button_info: ButtonMould{
            color: (150, 120, 200),
            hotkey: VirtualKeyCode::B
        },
        range: 0.0,
        casting_time: 500.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::BAKE_DOUGH_LAUNCHER, AbilityMould{
        cost: 500.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::DOUGH_LAUNCHER))),
        button_info: ButtonMould{
            color: (150, 120, 200),
            hotkey: VirtualKeyCode::L
        },
        range: 0.0,
        casting_time: 1000.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::WEP_BREAD, AbilityMould{
        cost: 0.0,
        targetting: AbilityTargetType::Unit(EffectUnitToUnit::LAUNCH_PROJECTILE(ProjectileMould{
            actor: ActorMould {
                colour: (10, 100, 20),
                size: PointFloat::new(5.0,5.0)
            },
            speed: 0.50,
            hit_effect: EffectToUnit::DAMAGE(EffectToUnitDamage{
                amount: 25.0
            })
        })),
        button_info: ButtonMould{
            color: (255, 0, 0),
            hotkey: VirtualKeyCode::Minus
        },
        range: 400.0,
        casting_time: 0.0,
        cooldown: 4000.0
    });
    data.abilities.insert(AbilityID::WEP_DOUGH_LAUNCHER, AbilityMould{
        cost: 0.0,
        targetting: AbilityTargetType::Unit(EffectUnitToUnit::LAUNCH_PROJECTILE(ProjectileMould{
            actor: ActorMould {
                colour: (200, 0, 0),
                size: PointFloat::new(25.0,25.0)
            },
            speed: 2.0,
            hit_effect: EffectToUnit::EFFECT_TO_POINT(EffectToPoint::EFFECT_NEARBY_UNITS(
                Box::new(EffectToUnit::DAMAGE(EffectToUnitDamage{
                    amount: 50.0
                })), 100.0))
        })),
        button_info: ButtonMould{
            color: (255, 0, 0),
            hotkey: VirtualKeyCode::Minus
        },
        range: 3000.0,
        casting_time: 0.0,
        cooldown: 5000.0
    });
}