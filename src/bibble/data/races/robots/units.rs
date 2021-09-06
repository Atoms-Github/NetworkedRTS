use crate::bibble::data::data_types::*;
use nalgebra::Point2;
use crate::pub_types::PointFloat;

pub fn units(data: &mut GameData){
    data.units.insert(UnitID::SCUTTLER, UnitMould{
        radius: 20.0,
        actor: ActorMould { image: "robot_spider.png".to_string(), size: PointFloat::new(40.0,40.0) },
        weapons: vec![AbilityID::WEP_SCUTTLER],
        abilities: vec![AbilityID::WEP_SCUTTLER],
        unit_flavour: UnitFlavour::HIKER,
        move_speed: 0.15,
        periodic_gain: Default::default(),
        life: 100.0
    });
    data.units.insert(UnitID::CONSTRUCTOR, UnitMould{
        radius: 15.0,
        actor: ActorMould { image: "robot_worker.jpg".to_string(), size: PointFloat::new(30.0, 30.0) },
        weapons: vec![],
        abilities: vec![AbilityID::BUILD_FOUNDRY],
        unit_flavour: UnitFlavour::HIKER,
        move_speed: 0.05,
        periodic_gain: Default::default(),
        life: 50.0
    });


    data.units.insert(
        UnitID::FOUNDRY,
        UnitMould{
            radius: 35.0,
            actor: ActorMould { image: "factory.jpg".to_string(), size: PointFloat::new(70.0, 70.0) },
            weapons: vec![],
            abilities: vec![AbilityID::TRAIN_SCUTTLER, AbilityID::TRAIN_CONSTRUCTOR],
            unit_flavour: UnitFlavour::STRUCTURE(StructureFlavourInfo{
                footprint: Point2::new(2,3)
            }),
            move_speed: 0.0,
            periodic_gain: ResourceBlock{
                resource_counts: [0.0, 0.0, 0.003]
            },
            life: 200.0
        }
    );

}