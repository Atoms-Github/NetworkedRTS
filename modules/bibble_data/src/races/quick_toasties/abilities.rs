use ggez::event::KeyCode;
use ggez::winit::event::VirtualKeyCode;
use crate::*;

pub fn abilities(data: &mut GameData){
    data.abilities.insert(AbilityID::BAKE_DOUGH, AbilityMould{
        cost: 15.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::DOUGH))),
        button_info: ButtonMould{
            color: (150, 120, 200),
            hotkey: KeyCode::D
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
            hotkey: KeyCode::B
        },
        range: 0.0,
        casting_time: 800.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::BAKE_DOUGH_LAUNCHER, AbilityMould{
        cost: 500.0,
        targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::DOUGH_LAUNCHER))),
        button_info: ButtonMould{
            color: (150, 120, 200),
            hotkey: KeyCode::L
        },
        range: 0.0,
        casting_time: 1000.0,
        cooldown: 0.0
    });
    data.abilities.insert(AbilityID::WEP_BREAD, AbilityMould{
        cost: 0.0,
        targetting: AbilityTargetType::SingleTarget(AbilitySingleTarget{
            target: AbilitySingleTargetType::Unit(EffectUnitToUnit::LAUNCH_SEEKING_PROJECTILE(SeekingProjectileMould {
                actor: ActorMould {
                    image: "energy_ball_yellow.png".to_string(),
                },
                speed: 1.0,
                hit_effect: EffectToUnit::DAMAGE(EffectToUnitDamage{
                    amount: 25.0
                }),
                size: 20.0
            })),
            graphic: AbilitySingleTargetGraphic::NOTHING
        }),
        button_info: ButtonMould{
            color: (255, 0, 0),
            hotkey: KeyCode::Minus
        },
        range: 400.0,
        casting_time: 0.0,
        cooldown: 1000.0
    });
    data.abilities.insert(AbilityID::WEP_DOUGH_LAUNCHER, AbilityMould{
        cost: 0.0,
        targetting: AbilityTargetType::SingleTarget(AbilitySingleTarget{
            target: AbilitySingleTargetType::Unit(EffectUnitToUnit::LAUNCH_SEEKING_PROJECTILE(SeekingProjectileMould {
                actor: ActorMould {
                    image: "butter.png".to_string(),
                },
                speed: 1.5,
                hit_effect: EffectToUnit::EFFECT_TO_POINT(EffectToPoint::EFFECT_NEARBY_UNITS(Box::new(
                    EffectToUnit::DAMAGE(EffectToUnitDamage{amount:50.0})
                ), 20.0)),
                size: 20.0
            })),
            graphic: AbilitySingleTargetGraphic::NOTHING
        }),
        button_info: ButtonMould{
            color: (255, 0, 0),
            hotkey: KeyCode::Minus
        },
        range: 3000.0,
        casting_time: 0.0,
        cooldown: 100.0
    });
}