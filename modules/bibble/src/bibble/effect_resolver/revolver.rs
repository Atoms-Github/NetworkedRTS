use crate::*;
use crate::bibble::data::data_types::{GameData, AbilityID, AbilitySingleTargetType};
use crate::bibble::{TechTreeComp, AbilityTargetInstance, OwnedComp};
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
        let mut target_point = None;
        let mut target_unit = None;
        match ability_target{
            AbilityTargetInstance::NO_TARGET => {}
            AbilityTargetInstance::UNIT(unit) => {target_unit = Some(unit)}
            AbilityTargetInstance::POINT(point) => {target_point = Some(point)},
        }


        let ability = data.get_ability(ability_id);
        match &ability.targetting{
            AbilityTargetType::NoTarget(effect_no_target) => {
                assert_eq!(ability_target, AbilityTargetInstance::NO_TARGET);
                self.revolve_to_unit(data, effect_no_target, source_unit);
            }
            AbilityTargetType::SingleTarget(single_target) => {
                match &single_target.target{
                    AbilitySingleTargetType::Unit(effect_unit_to_unit) => {
                        self.revolve_unit_to_unit(data, effect_unit_to_unit, source_unit, target_unit.unwrap());
                    }
                    AbilitySingleTargetType::Point(effect_unit_to_point) => {
                        self.revolve_unit_to_point(data, effect_unit_to_point, source_unit, &target_point.unwrap());
                    }
                    AbilitySingleTargetType::Plot(effect_unit_to_plot) => {
                        // Just do plot same as point.
                        self.revolve_unit_to_point(data, effect_unit_to_plot, source_unit, &target_point.unwrap());
                    }
                }
            }
        }
    }
}
