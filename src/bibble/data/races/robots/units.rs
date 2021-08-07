use crate::bibble::data::data_types::*;

pub fn units(data: &mut GameData){
    data.units.insert(UnitID::SCUTTLER, UnitMould{
        radius: 30.0,
        actor: ActorMould { colour: (100, 0, 100) },
        weapons: vec![WeaponID::GLAIVES]
    });
    data.units.insert(UnitID::CONSTRUCTOR, UnitMould{
        radius: 20.0,
        actor: ActorMould { colour: (100, 0, 200) },
        weapons: vec![]
    });
}