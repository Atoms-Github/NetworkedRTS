use crate::bibble::data::data_types::*;
use nalgebra::Point2;
use crate::pub_types::PointFloat;

pub fn units(data: &mut GameData){
    data.units.insert(UnitID::SCUTTLER, UnitMould{
        radius: 20.0,
        actor: ActorMould { colour: (100, 0, 100), size: PointFloat::new(40.0,40.0) },
        weapons: vec![AbilityID::WEP_SCUTTLER],
        abilities: vec![AbilityID::WEP_SCUTTLER],
        unit_flavour: UnitFlavour::HIKER,
        unit_cost: 1000
    });
    data.units.insert(UnitID::CONSTRUCTOR, UnitMould{
        radius: 15.0,
        actor: ActorMould { colour: (100, 0, 200), size: PointFloat::new(30.0, 30.0) },
        weapons: vec![],
        abilities: vec![],
        unit_flavour: UnitFlavour::HIKER,
        unit_cost: 300
    });


    data.units.insert(
        UnitID::FOUNDRY,
        UnitMould{
            radius: 35.0,
            actor: ActorMould { colour: (200, 60, 60), size: PointFloat::new(70.0,70.0) },
            weapons: vec![],
            abilities: vec![AbilityID::SPAWN_SCUTTLER],
            unit_flavour: UnitFlavour::STRUCTURE(StructureFlavourInfo{
                footprint: Point2::new(2,3)
            }),
            unit_cost: 2000
        }
    );

}