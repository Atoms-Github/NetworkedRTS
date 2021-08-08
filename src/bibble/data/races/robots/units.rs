use crate::bibble::data::data_types::*;
use nalgebra::Point2;

pub fn units(data: &mut GameData){
    data.units.insert(UnitID::SCUTTLER, UnitMould{
        radius: 30.0,
        actor: ActorMould { colour: (100, 0, 100) },
        weapons: vec![WeaponID::GLAIVES],
        unit_flavour: UnitFlavour::HIKER,
        unit_cost: 1000
    });
    data.units.insert(UnitID::CONSTRUCTOR, UnitMould{
        radius: 20.0,
        actor: ActorMould { colour: (100, 0, 200) },
        weapons: vec![],
        unit_flavour: UnitFlavour::HIKER,
        unit_cost: 300
    });


    data.units.insert(UnitID::FOUNDRY, UnitMould{
        radius: 20.0,
        actor: ActorMould { colour: (100, 255, 200) },
        weapons: vec![],
        unit_flavour: UnitFlavour::STRUCTURE(StructureFlavourInfo{
            footprint: Point2::new(2,3)
        }),
        unit_cost: 2000
    });

}