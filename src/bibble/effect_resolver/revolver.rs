use crate::ecs::superb_ecs::EntStructureChanges;
use crate::ecs::comp_store::CompStorage;
use crate::bibble::data::data_types::GameData;

pub struct Revolver<'a>{
    pub changes: EntStructureChanges,
    pub data: &'a GameData,
    pub c: &'a CompStorage
}

impl<'a> Revolver<'a>{
    pub fn new(c: &'a CompStorage, data: &'a GameData) -> Self{
        Self{
            changes: EntStructureChanges { new_entities: vec![], deleted_entities: vec![] },
            data,
            c
        }
    }
    pub fn end(self) -> EntStructureChanges{
        return self.changes;
    }
}
