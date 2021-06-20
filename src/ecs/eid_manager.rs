use std::collections::BTreeSet;
use crate::utils::TypeIdNum;
use crate::ecs::unmoving_vec::UnmovingVec;
use anymap::AnyMap;
use serde::*;
use std::convert::TryInto;
use crate::ecs_new_dawn::comp_store::{InternalEntity};


pub type GlobalEntityID = usize;
pub const MAX_ENTITIES :usize = 2;




#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct GlorifiedHashMap {
    alive: [bool; MAX_ENTITIES],
    entity_ids: [GlobalEntityID; MAX_ENTITIES],
    internal_details: [InternalEntity; MAX_ENTITIES],
}
impl Default for GlorifiedHashMap{
    fn default() -> Self {
        Self::new()
    }
}

impl GlorifiedHashMap {
    pub fn new() -> Self{
        let mut entity_ids = vec![];
        let mut internal_details = vec![];
        for i in 0..MAX_ENTITIES{
            entity_ids.push(i);
            internal_details.push(InternalEntity::default());
        }
        Self{
            alive: [false; MAX_ENTITIES],
            entity_ids: entity_ids.as_slice().try_into().unwrap(),
            internal_details: internal_details.as_slice().try_into().unwrap(),
        }
    }
    pub fn create_entity(&mut self, internal_entity: InternalEntity) -> GlobalEntityID{
        for index in 0..MAX_ENTITIES{
            if !self.alive[index]{
                self.alive[index] = true;
                let new_id = self.entity_ids[index] + MAX_ENTITIES;
                self.entity_ids[index] = new_id;
                self.internal_details[index] = internal_entity;
                return new_id;
            }
        }
        panic!("Exceeded entity storage capacity! Increase MAX_ENTITIES.");
    }
    pub fn delete(&mut self, query_id: GlobalEntityID) -> Option<&InternalEntity>{
        let index = query_id % MAX_ENTITIES;
        // If an entity lives in the spot.
        if self.alive[index]{
            // If the correct generation.
            if self.entity_ids[index] == query_id{
                self.alive[index] = false;
            }
        }
        return None;
    }
    pub fn get(&self, query_id: GlobalEntityID) -> Option<&InternalEntity>{
        let index = query_id % MAX_ENTITIES;
        // If an entity lives in the spot.
        if self.alive[index]{
            // If the correct generation.
            if self.entity_ids[index] == query_id{
                return Some(&self.internal_details[index]);
            }
        }
        return None;
    }
}


#[cfg(test)]
mod tests {
    use crate::ecs::eid_manager::*;

    #[test]
    fn basic() {
        // let comp1 = InternalEntity{
        //     global_id: 3,
        //     composition_id: 2,
        //     internal_index: 1
        // };
        // let comp2 = InternalEntity{
        //     global_id: 5,
        //     composition_id: 4,
        //     internal_index: 3
        // };
        // let mut storage = GlorifiedHashMap::new();
        // let id1 = storage.create_entity(comp1);
        // let id2 = storage.create_entity(comp2);
        // storage.delete(id1);
        // assert_eq!(comp2, *storage.get(id2).unwrap());

    }
}

