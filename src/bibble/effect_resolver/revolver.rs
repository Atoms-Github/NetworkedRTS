use crate::ecs::superb_ecs::EntStructureChanges;
use crate::ecs::comp_store::CompStorage;
use crate::bibble::data::data_types::GameData;
use crate::ecs::GlobalEntityID;
use crate::rts::compsys::{MyGlobalEntityID, TechTreeComp};

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
}
