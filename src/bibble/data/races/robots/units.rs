use crate::bibble::data::data_types::*;
use nalgebra::Point2;
use crate::pub_types::PointFloat;

pub fn units(data: &mut GameData){
    data.units.insert(UnitID::SCUTTLER, UnitMould{
        radius: 20.0,
        actor: ActorMould { image: "robot_spider.png".to_string(), },
        weapons: vec![AbilityID::WEP_SCUTTLER],
        abilities: vec![AbilityID::WEP_SCUTTLER],
        unit_flavour: UnitFlavour::HIKER(HikerFlavourInfo{
            movespeed: 0.15,
            fly: false
        }),
        periodic_gain: Default::default(),
        life: 100.0
    });
    data.units.insert(UnitID::CONSTRUCTOR, UnitMould{
        radius: 15.0,
        actor: ActorMould { image: "robot_worker.jpg".to_string(), },
        weapons: vec![],
        abilities: vec![AbilityID::BUILD_FOUNDRY],
        unit_flavour: UnitFlavour::HIKER(HikerFlavourInfo{
            movespeed: 0.08,
            fly: false
        }),
        periodic_gain: Default::default(),
        life: 50.0
    });


    data.units.insert(
        UnitID::FOUNDRY,
        UnitMould{
            radius: 35.0,
            actor: ActorMould { image: "factory.jpg".to_string(), },
            weapons: vec![],
            abilities: vec![AbilityID::TRAIN_SCUTTLER, AbilityID::TRAIN_CONSTRUCTOR],
            unit_flavour: UnitFlavour::STRUCTURE(StructureFlavourInfo{
                footprint: Point2::new(2,3)
            }),
            periodic_gain: ResourceBlock{
                resource_counts: [0.0, 0.0, 0.003]
            },
            life: 200.0
        }
    );

}