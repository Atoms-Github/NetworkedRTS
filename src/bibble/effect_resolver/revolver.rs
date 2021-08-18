use crate::ecs::superb_ecs::EntStructureChanges;
use crate::ecs::comp_store::CompStorage;
use crate::bibble::data::data_types::{GameData, AbilityID};
use crate::ecs::GlobalEntityID;
use crate::rts::compsys::{MyGlobalEntityID, TechTreeComp, AbilityTargetInstance, OwnedComp};
use crate::bibble::data::data_types::AbilityTargetType;

pub struct Revolver<'a>{
    pub changes: EntStructureChanges,
    pub c: &'a CompStorage
}

impl<'a> Revolver<'a>{
    pub fn new(c: &'a CompStorage) -> Self{
        Self{
            changes: EntStructureChanges { new_entities: vec![], deleted_entities: vec![] },
            c
        }
    }
    pub fn end(self) -> EntStructureChanges{
        return self.changes;
    }

    pub fn revolve_ability_execution(&mut self, data: &GameData, source_unit: GlobalEntityID,
                                     ability_id: AbilityID, ability_target: AbilityTargetInstance){
        let ability = data.get_ability(ability_id);
        match &ability.targetting{
            AbilityTargetType::NoTarget(effect_no_target) => {
                match ability_target{
                    AbilityTargetInstance::NO_TARGET => {
                        self.revolve_to_unit(data, effect_no_target, source_unit);
                    }
                    _ => {panic!("Wrong target type.")}
                }
            }
            AbilityTargetType::Unit(effect_unit_to_unit) => {
                match ability_target{
                    AbilityTargetInstance::UNIT(target_unit) => {
                        self.revolve_unit_to_unit(data, effect_unit_to_unit, source_unit, target_unit);
                    }
                    _ => {panic!("Wrong target type.")}
                }
            }
            AbilityTargetType::Point(effect_to_point) => {
                match ability_target{
                    AbilityTargetInstance::POINT(target_point) => {
                        self.revolve_unit_to_point(data, effect_to_point, source_unit, target_point);
                    }
                    _ => {panic!("Wrong target type.")}
                }
            }
        }
    }
}
