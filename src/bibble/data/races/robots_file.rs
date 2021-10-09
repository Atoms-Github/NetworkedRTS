use crate::bibble::data::data_types::GameData;
use nalgebra::{Point, Point2};
use serde::*;
use crate::pub_types::PointFloat;
use crate::rts::compsys::{ResourceBlock, PlotFlooring};
use crate::bibble::data::data_types::*;
use serde::de::Unexpected::Float;

pub fn gather(data: &mut GameData){
    // Race.
    {
        let mut starting_effects = vec![];

        for _ in 0..1{
            starting_effects.push(EffectToPoint::SPAWN_UNIT(UnitID::CONSTRUCTOR));
        }
        for _ in 0..1{
            starting_effects.push(EffectToPoint::SPAWN_UNIT(UnitID::ROBO_SPIDER));
        }
        for _ in 0..1{
            starting_effects.push(EffectToPoint::SPAWN_UNIT(UnitID::OIL_WELL));
        }

        data.races.insert(RaceID::ROBOTS, RaceMould{
            spawn_effect: EffectToPoint::COMPOSITE(starting_effects),
            icon: "robot_icon.jpg".to_string()
        });
    }
    {
        let unit = data.add_unit(UnitID::ROBO_SPIDER, UnitMould{
            radius: 20.0,
            actor: ActorMould { image: "robot_spider.png".to_string(), },
            weapons: vec![],
            abilities: vec![],
            unit_flavour: UnitFlavour::HIKER(HikerFlavourInfo{
                movespeed: 0.15,
                fly: false
            }),
            periodic_gain: Default::default(),
            life: 100.0
        });
        data.add_weapon(unit, AbilityID::WEP_ROBO_SPIDER, EffectUnitToUnit::LAUNCH_SEEKING_PROJECTILE(SeekingProjectileMould {
            actor: ActorMould {
                image: "robot_spider_projectile.png".to_string(),
            },
            speed: 0.5,
            hit_effect: EffectToUnit::DAMAGE(EffectToUnitDamage{
                amount: 10.0
            }),
            size: 20.0
        }), 200.0, 300.0);
    }
    {
        let mut unit = data.add_unit(UnitID::CONSTRUCTOR, UnitMould{
            radius: 15.0,
            actor: ActorMould { image: "robot_worker.jpg".to_string(), },
            weapons: vec![],
            abilities: vec![AbilityID::BUILD_FACTORY, AbilityID::BUILD_OIL_WELL],
            unit_flavour: UnitFlavour::HIKER(HikerFlavourInfo{
                movespeed: 0.08,
                fly: false
            }),
            periodic_gain: Default::default(),
            life: 50.0
        });
        data.abilities.insert(AbilityID::BUILD_OIL_WELL, AbilityMould{
            cost: 15.0,
            targetting: AbilityTargetType::SingleTarget(AbilitySingleTarget{
                target: AbilitySingleTargetType::Plot(EffectUnitToPoint::TO_POINT(EffectToPoint::BUILD_BUILDING(UnitID::OIL_WELL))),
                graphic: AbilitySingleTargetGraphic::NOTHING
            }),
            button_info: ButtonMould{
                color: (150, 255, 130),
                hotkey: VirtualKeyCode::R
            },
            range: 10.0,
            casting_time: 2000.0,
            cooldown: 0.0
        });
        data.abilities.insert(AbilityID::BUILD_FACTORY, AbilityMould{
            cost: 60.0,
            targetting: AbilityTargetType::SingleTarget(AbilitySingleTarget{
                target: AbilitySingleTargetType::Plot(EffectUnitToPoint::TO_POINT(EffectToPoint::BUILD_BUILDING(UnitID::FACTORY))),
                graphic: AbilitySingleTargetGraphic::NOTHING
            }),
            button_info: ButtonMould{
                color: (150, 120, 200),
                hotkey: VirtualKeyCode::F
            },
            range: 0.0,
            casting_time: 2000.0,
            cooldown: 0.0
        });
    }
    {
        let mut unit = data.add_unit(UnitID::FACTORY, UnitMould{
            radius: 35.0,
            actor: ActorMould { image: "factory.jpg".to_string(), },
            weapons: vec![],
            abilities: vec![AbilityID::TRAIN_SCUTTLER, AbilityID::TRAIN_CONSTRUCTOR],
            unit_flavour: UnitFlavour::STRUCTURE(StructureFlavourInfo{
                footprint: Point2::new(2,2),
                required_under_material: PlotFlooring::PATH,
            }),
            periodic_gain: ResourceBlock{
                resource_counts: [0.0, 0.0, 0.0]
            },
            life: 200.0
        });
        data.abilities.insert(AbilityID::TRAIN_SCUTTLER, AbilityMould{
            cost: 30.0,
            targetting: AbilityTargetType::NoTarget(EffectToUnit::EFFECT_TO_POINT(EffectToPoint::SPAWN_UNIT(UnitID::ROBO_SPIDER))),
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
    }
    {
        let mut unit = data.add_unit(UnitID::OIL_WELL, UnitMould{
            radius: 20.0,
            actor: ActorMould { image: "robot_oil_well.png".to_string(), },
            weapons: vec![],
            abilities: vec![],
            unit_flavour: UnitFlavour::STRUCTURE(StructureFlavourInfo{
                footprint: Point2::new(1,1),
                required_under_material: PlotFlooring::GREEN_RESOURCE,
            }),
            periodic_gain: ResourceBlock{
                resource_counts: [0.0, 0.0, 1.003]
            },
            life: 100.0
        });
    }


}












