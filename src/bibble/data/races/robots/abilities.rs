use crate::bibble::data::data_types::*;
use nalgebra::Point2;
use winit::VirtualKeyCode;
use crate::pub_types::PointFloat;

pub fn abilities(data: &mut GameData){
    data.abilities.insert(AbilityID::BUILD_FOUNDRY, AbilityMould{
        cost: 60.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::FOUNDRY))),
        button_info: ButtonMould{
            color: (150, 120, 200),
            hotkey: VirtualKeyCode::F
        },
        range: 0.0,
        casting_time: 2000.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::TRAIN_SCUTTLER, AbilityMould{
        cost: 30.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::SCUTTLER))),
        button_info: ButtonMould{
            color: (150, 120, 200),
            hotkey: VirtualKeyCode::S
        },
        range: 0.0,
        casting_time: 1000.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::TRAIN_CONSTRUCTOR, AbilityMould{
        cost: 15.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::CONSTRUCTOR))),
        button_info: ButtonMould{
            color: (150, 120, 200),
            hotkey: VirtualKeyCode::C
        },
        range: 0.0,
        casting_time: 500.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::WEP_SCUTTLER, AbilityMould{
        cost: 0.0,
        targetting: AbilityTargetType::Unit(EffectUnitToUnit::LAUNCH_PROJECTILE(ProjectileMould{
            actor: ActorMould {
                image: "robot_spider_projectile.png".to_string(),
                size: PointFloat::new(10.0,10.0)
            },
            speed: 0.25,
            hit_effect: EffectToUnit::DAMAGE(EffectToUnitDamage{
                amount: 10.0
            })
        })),
        button_info: ButtonMould{
            color: (255, 0, 0),
            hotkey: VirtualKeyCode::Minus
        },
        range: 200.0,
        casting_time: 0.0,
        cooldown: 300.0
    });

}