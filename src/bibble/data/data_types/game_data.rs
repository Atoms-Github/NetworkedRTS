use std::collections::BTreeMap;
use super::*;

pub struct GameData{
    pub units: BTreeMap<UnitID, UnitMould>,
    pub weapons: BTreeMap<WeaponID, WeaponMould>,
    pub races: BTreeMap<RaceID, RaceMould>,
    pub projectiles: BTreeMap<ProjectileID, ProjectileMould>,
    pub actors: BTreeMap<ActorID, ActorMould>,
}


impl GameData{
    pub fn add_weapon(&mut self, id: WeaponID, mould: WeaponMould){
        data.weapons.insert(id, mould);
    }
    pub fn get_weapon(&mut self, id: WeaponID) -> &WeaponMould{
        return data.weapons.get(&id).unwrap();
    }
}