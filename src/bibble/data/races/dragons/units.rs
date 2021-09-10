use crate::bibble::data::data_types::*;
use nalgebra::Point2;
use crate::pub_types::PointFloat;

pub fn units(data: &mut GameData){
    data.units.insert(UnitID::RED_DRAGON, UnitMould{
        radius: 40.0,
        actor: ActorMould { image: "red_dragon.jpg".to_string(), },
        weapons: vec![AbilityID::WEP_RED_DRAGON],
        abilities: vec![AbilityID::WEP_RED_DRAGON],
        unit_flavour: UnitFlavour::HIKER,
        move_speed: 0.3,
        fly: true,
        periodic_gain: Default::default(),
        life: 300.0
    });
    data.units.insert(UnitID::RED_DRAGON_EGG, UnitMould{
        radius: 15.0,
        actor: ActorMould { image: "red_dragon_egg.jpg".to_string(), },
        weapons: vec![],
        abilities: vec![],
        unit_flavour: UnitFlavour::HIKER,
        move_speed: 0.00,
        fly: false,
        periodic_gain: Default::default(),
        life: 20.0
    });
    data.units.insert(UnitID::SMALL_DRAGON, UnitMould{
        radius: 20.0,
        actor: ActorMould { image: "small_dragon.jpg".to_string(), },
        weapons: vec![AbilityID::WEP_SMALL_DRAGON],
        abilities: vec![AbilityID::WEP_SMALL_DRAGON],
        unit_flavour: UnitFlavour::HIKER,
        move_speed: 0.1,
        fly: true,
        periodic_gain: Default::default(),
        life: 50.0
    });
    data.units.insert(
        UnitID::VOLCANO,
        UnitMould{
            radius: 40.0,
            actor: ActorMould { image: "volcano.jpg".to_string(), },
            weapons: vec![],
            abilities: vec![AbilityID::TRAIN_SMALL_DRAGON, AbilityID::TRAIN_RED_DRAGON_EGG],
            unit_flavour: UnitFlavour::STRUCTURE(StructureFlavourInfo{
                footprint: Point2::new(2,2)
            }),
            move_speed: 0.0,
            fly: false,
            periodic_gain: ResourceBlock{
                resource_counts: [0.0, 0.0, 0.005]
            },
            life: 400.0
        }
    );

}