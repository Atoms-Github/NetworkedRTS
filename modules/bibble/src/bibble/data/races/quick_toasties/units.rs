use crate::*;


pub fn units(data: &mut GameData){
    data.units.insert(UnitID::DOUGH_LAUNCHER, UnitMould{
        radius: 100.0,
        actor: ActorMould { image: "bread_cannon.jpg".to_string(), },
        weapons: vec![AbilityID::WEP_DOUGH_LAUNCHER],
        abilities: vec![AbilityID::WEP_DOUGH_LAUNCHER],
        unit_flavour: UnitFlavour::STRUCTURE(StructureFlavourInfo{
            footprint: Point2::new(4,4),
            required_under_material: Default::default()
        }),
        periodic_gain: ResourceBlock::default(),
        life: 500.0,
    });

    data.units.insert(UnitID::DOUGH, UnitMould{
        radius: 15.0,
        actor: ActorMould { image: "dough_ball.jpg".to_string(), },
        weapons: vec![],
        abilities: vec![AbilityID::BAKE_BREAD, AbilityID::BAKE_DOUGH, AbilityID::BAKE_DOUGH_LAUNCHER],
        unit_flavour: UnitFlavour::HIKER(HikerFlavourInfo{
            movespeed: 0.3,
            fly: false
        }),
        periodic_gain: ResourceBlock{
            resource_counts: [0.0, 0.0, 0.0003]
        },
        life: 30.0,
    });

    data.units.insert(
        UnitID::BREAD,
        UnitMould{
            radius: 25.0,
            actor: ActorMould { image: "bread.png".to_string(), },
            weapons: vec![AbilityID::WEP_BREAD],
            abilities: vec![AbilityID::WEP_BREAD],
            unit_flavour: UnitFlavour::HIKER(HikerFlavourInfo{
                movespeed: 0.1,
                fly: false
            }),
            periodic_gain: Default::default(),
            life: 200.0
        }
    );

}