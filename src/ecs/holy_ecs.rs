use std::any::{Any, TypeId};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

use anymap::any::CloneAny;
use anymap::AnyMap;
use serde::{Deserialize, Serialize, Serializer};

use crate::ecs::{System, GlobalEntityID};
use crate::ecs::ecs_shared::{SerdeObject, Component};
use crate::ecs::my_anymap::SerdeAnyMap;
use crate::ecs::systems_man::SystemsMan;
use crate::utils::{TypeIdNum};
use serde::de::DeserializeOwned;
use crate::ecs::liquid_garbage::TOH;
use crate::ecs::unmoving_vec::UnmovingVec;
use std::convert::TryInto;


pub type TypeSet = BTreeSet<TypeIdNum>;
pub type InternalIndex = usize;
pub type GenerationNum = usize;






#[derive(Serialize, Deserialize, Clone)]
struct InternalEntityHandle {
    index: InternalIndex,
    generation: GenerationNum,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HolyEcs {
    storages: BTreeMap<TypeIdNum, Vec<Option<TOH>>>,
    generations: Vec<GenerationNum>,
    spare_slots: UnmovingVec<()>,
    next_external_id: GlobalEntityID,
    external_entity_lookup: BTreeMap<GlobalEntityID, InternalEntityHandle>,
}
impl HolyEcs {
    pub fn new() -> Self{
        HolyEcs{
            storages: Default::default(),
            generations: vec![],
            spare_slots: Default::default(),
            next_external_id: 0,
            external_entity_lookup: Default::default()
        }
    }
    fn get_storage(&mut self, type_id: TypeIdNum) -> &mut Vec<Option<TOH>>{
        if !self.storages.contains_key(&type_id){
            self.storages.insert(type_id, vec![]);
            for internal_index in 0..self.generations.len(){
                self.storages.get_mut(&type_id).unwrap().push(None);
            }
        }
        return self.storages.get_mut(&type_id).unwrap();
    }

    fn generate_new_empty_entity(&mut self) -> GlobalEntityID{
        let internal_index = self.spare_slots.push(());
        let new_space = internal_index == self.spare_slots.capacity() - 1;
        if new_space {
            self.generations.push(0);
            for (type_id, component_list) in &mut self.storages{
                component_list.push(None);
            }
        }
        let new_generation = self.generations.get(internal_index).unwrap() + 1;
        self.generations[internal_index] = new_generation;

        let internal = InternalEntityHandle {
            index: internal_index,
            generation: new_generation,
        };
        let external_id = self.next_external_id;
        self.next_external_id += 1;
        self.external_entity_lookup.insert(external_id, internal);

        external_id
    }
    pub fn new_entity(&mut self, new_components: SerdeAnyMap) -> GlobalEntityID {
        let external_id = self.generate_new_empty_entity();
        let internal_entity_index = self.external_entity_lookup.get(&external_id).unwrap().index;

        for (type_id, component) in new_components.data{
            self.get_storage(type_id)[internal_entity_index] = Some(component);
        }
        return external_id;
    }
    pub fn add_component<T : Component + 'static>(&mut self, external_id: GlobalEntityID, component: T) {
        // If its a real entity.
        if let Some(internal_entity_original) = self.external_entity_lookup.get(&external_id).clone(){
            let internal_entity_cloned = internal_entity_original.clone();
            // If not dead.
            if *self.generations.get(internal_entity_cloned.index).unwrap() == internal_entity_cloned.generation{
                let my_type = crate::utils::crack_type_id::<T>();

                let found = self.get_storage(my_type).get_mut(internal_entity_cloned.index).unwrap();
                self.get_storage(my_type)[internal_entity_cloned.index] = Some(TOH::new(component));
            }
        }
    }

    pub fn query(&mut self, types: Vec<TypeIdNum>) -> Vec<GlobalEntityID> {
        let mut internal_entities = vec![];
        for internal_index in 0..self.generations.len(){
            // If alive.
            if self.spare_slots.get(internal_index).is_some(){
                let mut good = true;
                for required_type in &types{
                    if self.get_storage(*required_type).get(internal_index).unwrap().is_none(){
                        good = false;
                        break;
                    }
                }
                if good{
                    internal_entities.push(internal_index);
                }
            }
        }
        let mut external_entities = vec![];
        for internal_entity in &internal_entities{
            for (external_id, internal_handle) in &self.external_entity_lookup{
                if internal_handle.index == *internal_entity{
                    external_entities.push(external_id);
                    break;
                }
            }
        }
        return internal_entities;
    }

    pub fn get<T: Component + DeserializeOwned>(&mut self, external_id: GlobalEntityID) -> Option<&T> {
        match self.get_mut::<T>(external_id){
            Some(value) => {
                let moved :&T = value;
                return Some(moved);
            }
            None =>{
                None
            }
        }
    }
    pub fn get_mut<T: Component + DeserializeOwned>(&mut self, external_id: GlobalEntityID) -> Option<&mut T> {
        // If its a real entity.
        if let Some(internal_entity_original) = self.external_entity_lookup.get(&external_id).clone(){
            let internal_entity_cloned = internal_entity_original.clone();
            // If not dead.
            if *self.generations.get(internal_entity_cloned.index).unwrap() == internal_entity_cloned.generation{
                let my_type = crate::utils::crack_type_id::<T>();

                let found = self.get_storage(my_type).get_mut(internal_entity_cloned.index).unwrap();
                // If the entity does have that component.
                if let Some(toh) = found{
                    return Some(toh.get::<T>())
                }
            }
        }
        return None;
    }

    pub fn run_systems(&mut self, systems: &SystemsMan) {
        for system in &systems.systems{
            system.run(self);
        }
    }
}








#[cfg(test)]
mod ecs_tests {
    use super::*;
    const TEST_COMP_3_VALUE: usize = 3;

    #[derive(Clone, Serialize, Deserialize)]
    pub struct TestComp1 {
        value: usize
    }
    impl Component for TestComp1 {}
    impl SerdeObject for TestComp1 {
        fn my_clone(&self) -> Box<dyn SerdeObject> {
            Box::new(self.clone())
        }
        fn my_ser(&self) -> Vec<u8> {
            return bincode::serialize(self).unwrap();
        }
    }
    #[derive(Clone, Serialize, Deserialize)]
    pub struct TestComp2 {
        value: f32
    }
    impl Component for TestComp2 {}
    impl SerdeObject for TestComp2 {
        fn my_clone(&self) -> Box<dyn SerdeObject> {
            Box::new(self.clone())
        }
        fn my_ser(&self) -> Vec<u8> {
            return bincode::serialize(self).unwrap();
        }
    }
    #[derive(Clone, Serialize, Deserialize)]
    pub struct TestComp3 {
        value: usize
    }
    impl Component for TestComp3 {}
    impl SerdeObject for TestComp3 {
        fn my_clone(&self) -> Box<dyn SerdeObject> {
            Box::new(self.clone())
        }
        fn my_ser(&self) -> Vec<u8> {
            return bincode::serialize(self).unwrap();
        }
    }
    fn new_entity() -> (HolyEcs, GlobalEntityID){
        let mut ecs = HolyEcs::new();
        let mut new_entity_comps = SerdeAnyMap::new();
        new_entity_comps.insert(TestComp3{value : TEST_COMP_3_VALUE});
        new_entity_comps.insert(TestComp2{value : 3.2});

        let new_entity_id = ecs.new_entity(new_entity_comps);
        assert_eq!(new_entity_id, 0);
        return (ecs, new_entity_id);
    }
    #[test]
    fn ecs_new_entity() {
        new_entity();
    }
    #[test]
    fn ecs_query_positive() {
        let (mut ecs, entity_id) = new_entity();
        let query_results = ecs.query(vec![crate::utils::crack_type_id::<TestComp2>()]);
        assert_eq!(1, query_results.len());
        assert_eq!(entity_id, *query_results.get(0).unwrap());
    }
    #[test]
    fn ecs_query_negative() {
        let (mut ecs, entity_id) = new_entity();
        let query_results = ecs.query(vec![crate::utils::crack_type_id::<TestComp1>(), crate::utils::crack_type_id::<TestComp3>()]);
        assert_eq!(0, query_results.len());
    }
    #[test]
    fn ecs_get_comp() {
        let (mut ecs, entity_id) = new_entity();
        let value = ecs.get_mut::<TestComp3>(entity_id).unwrap();
        assert_eq!(value.value, TEST_COMP_3_VALUE);
    }

}












// #[derive(Serialize, Deserialize, Clone)]
// struct Column<T : Component + 'static>{
//     vec: Vec<Option<T>>,
// }
// impl<T : Component + 'static> SerdeObject for Column<T>{
//     fn my_clone(&self) -> Box<dyn SerdeObject> {
//         Box::new(self.clone())
//     }
//     fn my_ser(&self) -> Vec<u8> {
//         return bincode::serialize(self).unwrap();
//     }
// }