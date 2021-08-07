use std::collections::BTreeMap;
use super::*;

pub struct GameData{
    pub units: BTreeMap<UnitID, UnitMould>,
    pub weapons: BTreeMap<WeaponID, WeaponMould>,
    pub races: BTreeMap<RaceID, RaceMould>,
    pub projectiles: BTreeMap<ProjectileID, ProjectileMould>,
}


impl GameData{
    pub fn get_weapon(&self, id: WeaponID) -> &WeaponMould{
        return self.weapons.get(&id).unwrap();
    }
    pub fn get_race(&self, id: RaceID) -> &RaceMould{
        return self.races.get(&id).unwrap();
    }
}