use crate::bibble::data::data_types::*;
use nalgebra::Point2;
use crate::pub_types::PointFloat;


pub fn units(data: &mut GameData){
    data.units.insert(UnitID::DOUGH_LAUNCHER, UnitMould{
        radius: 100.0,
        actor: ActorMould { colour: (217, 180, 48), size: PointFloat::new(200.0,200.0) },
        weapons: vec![AbilityID::WEP_DOUGH_LAUNCHER],
        abilities: vec![AbilityID::WEP_DOUGH_LAUNCHER],
        unit_flavour: UnitFlavour::STRUCTURE(StructureFlavourInfo{
            footprint: Point2::new(3,3)
        }),
        move_speed: 0.00,
        periodic_gain: ResourceBlock::default(),
        life: 500.0,
    });

    data.units.insert(UnitID::DOUGH, UnitMould{
        radius: 10.0,
        actor: ActorMould { colour: (217, 180, 48), size: PointFloat::new(20.0,20.0) },
        weapons: vec![],
        abilities: vec![AbilityID::BAKE_BREAD, AbilityID::BAKE_DOUGH, AbilityID::BAKE_DOUGH_LAUNCHER],
        unit_flavour: UnitFlavour::HIKER,
        move_speed: 0.30,
        periodic_gain: ResourceBlock{
            resource_counts: [0.0, 0.0, 0.001]
        },
        life: 30.0,
    });

    data.units.insert(
        UnitID::BREAD,
        UnitMould{
            radius: 25.0,
            actor: ActorMould { colour: (217, 107, 48), size: PointFloat::new(50.0,50.0) },
            weapons: vec![AbilityID::WEP_BREAD],
            abilities: vec![AbilityID::WEP_BREAD],
            unit_flavour: UnitFlavour::HIKER,
            move_speed: 0.10,
            periodic_gain: Default::default(),
            life: 200.0
        }
    );

}