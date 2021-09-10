use std::collections::BTreeMap;
use serde::*;
use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct GameData{ // TODO: Rename :)
    pub units: BTreeMap<UnitID, UnitMould>,
    pub weapons: BTreeMap<WeaponID, WeaponMould>,
    pub races: BTreeMap<RaceID, RaceMould>,
    pub abilities: BTreeMap<AbilityID, AbilityMould>,
}


impl GameData{
    pub fn get_weapon(&self, id: WeaponID) -> &WeaponMould{
        return self.weapons.get(&id).unwrap();
    }
    pub fn get_race(&self, id: RaceID) -> &RaceMould{
        return self.races.get(&id).unwrap();
    }
    pub fn get_ability(&self, id: AbilityID) -> &AbilityMould{
        if let Some(ability) = self.abilities.get(&id){
            return ability;
        }else{
            panic!("Can't find ability {:?}", id);
        }
    }
}