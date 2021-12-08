use std::collections::BTreeMap;
use serde::*;
use super::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameData{ // TODO: Rename :)
    pub units: BTreeMap<UnitID, UnitMould>,
    pub races: BTreeMap<RaceID, RaceMould>,
    pub abilities: BTreeMap<AbilityID, AbilityMould>,
}

// For runtime.
impl GameData{
    pub fn get_unit(&self, id: UnitID) -> &UnitMould{
        return self.units.get(&id).unwrap();
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
// For data definition.
impl GameData{
    pub fn add_unit(&mut self, id: UnitID, mould: UnitMould) -> UnitID{
        self.units.insert(id, mould);
        return id;
    }
    pub fn add_weapon(&mut self, unit: UnitID, id: AbilityID, effect: EffectUnitToUnit, range: f32, cooldown: f32){
        let unit = self.units.get_mut(&unit).unwrap();
        unit.weapons.push(id);
        unit.abilities.push(id);
        self.abilities.insert(id, AbilityMould{
            cost: 0.0,
            targetting: AbilityTargetType::SingleTarget(AbilitySingleTarget{
                target: AbilitySingleTargetType::Unit(effect),
                graphic: AbilitySingleTargetGraphic::NOTHING
            }),
            button_info: ButtonMould{
                color: (255, 0, 0),
                hotkey: VirtualKeyCode::Minus
            },
            range,
            casting_time: 0.0,
            cooldown,
        });
    }
}