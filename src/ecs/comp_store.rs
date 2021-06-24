use std::collections::{BTreeMap, BTreeSet};
use crate::utils::TypeIdNum;
use serde::*;
use crate::ecs::eid_manager::{GlorifiedHashMap, GlobalEntityID};
use anymap::AnyMap;
use crate::ecs::pending_entity::PendingEntity;

type ByteBlock = Vec<u8>;
type Column = Vec<Vec<ByteBlock>>;

pub type CompositionID = usize;
pub type GenerationNum = usize;
pub type InternalIndex = usize;
pub type TypesSet = BTreeSet<TypeIdNum>;

#[derive(Clone, Serialize, Default, Deserialize, Debug, Hash, Copy, PartialEq)]
pub struct InternalEntity {
    pub global_id: GlobalEntityID,
    composition_id: CompositionID,
    internal_index: InternalIndex,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CompStorage {
    columns: BTreeMap<TypeIdNum, Column>,
    entities: GlorifiedHashMap,
    composition_ids: BTreeMap<TypesSet, CompositionID>,
    next_composition_id: CompositionID,
    global_ids_as_comps: Vec<Vec<GlobalEntityID>>,
}
pub struct DeleteResult{

}
impl CompStorage{
    pub fn get<T : 'static>(&self, entity_id: GlobalEntityID) -> Option<&T>{
        // Pog. Can be pure 0 processing time function if we use get_unsafe().
        let internal = self.entities.get(entity_id)?;
        let column = self.get_column::<T>()?;
        let bytes = column.get(internal.composition_id)?.get(internal.internal_index)?;

        let casted : &T = unsafe{u8_slice_to_ref(bytes.as_slice())};
        return Some(casted);
    }
    pub fn get_mut<T : 'static>(&/*Non-mut. Unsafe loh.*/self, entity_id: GlobalEntityID) -> Option<&mut T>{
        return self.get::<T>(entity_id).map(|unmut|{unsafe{very_bad_function(unmut)}});
    }
    pub fn delete_entity(&mut self, entity_id: GlobalEntityID) -> Option<DeleteResult>{
        let deleting_internal = self.entities.get(entity_id)?;

        // Remember to also delete in global_ids_as_comps.
        unimplemented!();
    }
    pub fn get_composition_id(&mut self, types_hash: TypesSet) -> CompositionID{
        if let Some(composition_id) = self.composition_ids.get(&types_hash){
            return *composition_id;
        }
        let new_composition_id = self.next_composition_id;
        self.next_composition_id += 1;

        self.composition_ids.insert(types_hash, new_composition_id);

        return new_composition_id;
    }
    pub fn create_entity(&mut self, pending_entity: PendingEntity) -> InternalIndex{
        let composition_id = self.get_composition_id(pending_entity.hash_types());

        let mut internal_index : Option<InternalIndex> = None;
        for (type_id, bytes) in pending_entity.iter(){
            let block = self.get_block_or_make(*type_id, composition_id);
            let new_internal_index = block.len();
            block.push(bytes.clone());

            if let Some(existing_internal_index) = internal_index{
                assert_eq!(existing_internal_index, new_internal_index, "ECS internal indices for new entity were different!");
            }else{
                internal_index = Some(new_internal_index);
            }
        }

        let internal_entity = InternalEntity{
            global_id: 99999998,
            composition_id,
            internal_index: internal_index.expect("New entity had 0 components! This is disallowed."),
        };
        let global_entity_id = self.entities.create_entity(internal_entity);

        self.save_global_entity_id(composition_id, global_entity_id);


        return global_entity_id;
    }
    pub fn query(&self, must_include: Vec<TypeIdNum>) -> Vec<GlobalEntityID>{
        let mut found_entities = vec![];
        for type_set in self.composition_ids.keys(){
            let mut pass = true;
            for include_type in &must_include{
                if !type_set.contains(include_type){
                    pass = false;
                    break;
                }
            }

            // type_set matches our query.
            if pass{
                let composition_id = self.composition_ids.get(type_set).unwrap();
                found_entities.append(&mut self.global_ids_as_comps.get(*composition_id).unwrap().clone());
            }

        }
        return found_entities;
    }
    fn save_global_entity_id(&mut self, composition_id: CompositionID, global_id: GlobalEntityID){
        for new_block_index in self.global_ids_as_comps.len()..(composition_id + 1){
            self.global_ids_as_comps.push(vec![]);
        }
        self.global_ids_as_comps.get_mut(composition_id).unwrap().push(global_id);
    }
    fn get_block_or_make(&mut self, type_id: TypeIdNum, composition_id: CompositionID) -> &mut Vec<ByteBlock>{
        let column = self.get_column_mut_or_make_key(type_id);
        for new_block_index in column.len()..(composition_id + 1){
            column.push(vec![]);
        }
        return column.get_mut(composition_id).unwrap();
    }
    fn get_column<T : 'static>(&self) -> Option<&Column>{
        self.columns.get(&crate::utils::gett::<T>())
    }
    fn get_column_mut_or_make<T : 'static>(&mut self) -> &mut Column{
        let key = crate::utils::gett::<T>();
        return self.get_column_mut_or_make_key(key);
    }
    fn get_column_mut_or_make_key(&mut self, type_id: TypeIdNum) -> &mut Column{
        if self.columns.get(&type_id).is_none(){
            self.columns.insert(type_id, vec![]);
        }
        return self.columns.get_mut(&type_id).unwrap();
    }

}
unsafe fn very_bad_function<T>(reference: &T) -> &mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}
unsafe fn u8_slice_to_ref<T>(bytes: &[u8]) -> &T {
    let bytes_ptr = bytes.as_ptr();
    let test : *const T = unsafe{ std::mem::transmute(bytes_ptr) };
    let value = unsafe {test.as_ref()}.unwrap();
    return value;
}
unsafe fn u8_slice_to_ref_mut<T>(bytes: &mut [u8]) -> &mut T {
    let bytes_ptr = bytes.as_ptr();
    let test : *mut T = unsafe{ std::mem::transmute(bytes_ptr) };
    let value : &mut T = unsafe {test.as_mut()}.unwrap();
    return value;
}


// #[cfg(test)]
// mod ecs_tests {
//     use super::*;
//     const TEST_COMP_3_VALUE: usize = 3;
//
//     #[derive(Clone, Serialize, Deserialize)]
//     pub struct TestComp1 {
//         value: usize
//     }
//     impl Component for TestComp1 {}
//     impl SerdeObject for TestComp1 {
//         fn my_clone(&self) -> Box<dyn SerdeObject> {
//             Box::new(self.clone())
//         }
//         fn my_ser(&self) -> Vec<u8> {
//             return bincode::serialize(self).unwrap();
//         }
//     }
//     #[derive(Clone, Serialize, Deserialize)]
//     pub struct TestComp2 {
//         value: f32
//     }
//     impl Component for TestComp2 {}
//     impl SerdeObject for TestComp2 {
//         fn my_clone(&self) -> Box<dyn SerdeObject> {
//             Box::new(self.clone())
//         }
//         fn my_ser(&self) -> Vec<u8> {
//             return bincode::serialize(self).unwrap();
//         }
//     }
//     #[derive(Clone, Serialize, Deserialize)]
//     pub struct TestComp3 {
//         value: usize
//     }
//     impl Component for TestComp3 {}
//     impl SerdeObject for TestComp3 {
//         fn my_clone(&self) -> Box<dyn SerdeObject> {
//             Box::new(self.clone())
//         }
//         fn my_ser(&self) -> Vec<u8> {
//             return bincode::serialize(self).unwrap();
//         }
//     }
//     fn new_entity() -> (HolyEcs, GlobalEntityID){
//         let mut ecs = HolyEcs::new();
//         let mut new_entity_comps = SerdeAnyMap::new();
//         new_entity_comps.insert(TestComp3{value : TEST_COMP_3_VALUE});
//         new_entity_comps.insert(TestComp2{value : 3.2});
//
//         let new_entity_id = ecs.new_entity(new_entity_comps);
//         assert_eq!(new_entity_id, 0);
//         return (ecs, new_entity_id);
//     }
//     #[test]
//     fn ecs_new_entity() {
//         new_entity();
//     }
//     #[test]
//     fn ecs_query_positive() {
//         let (mut ecs, entity_id) = new_entity();
//         let query_results = ecs.query(vec![crate::utils::get_type_id::<TestComp2>()]);
//         assert_eq!(1, query_results.len());
//         assert_eq!(entity_id, *query_results.get(0).unwrap());
//     }
//     #[test]
//     fn ecs_query_negative() {
//         let (mut ecs, entity_id) = new_entity();
//         let query_results = ecs.query(vec![crate::utils::get_type_id::<TestComp1>(), crate::utils::get_type_id::<TestComp3>()]);
//         assert_eq!(0, query_results.len());
//     }
//     #[test]
//     fn ecs_get_comp() {
//         let (mut ecs, entity_id) = new_entity();
//         let value = ecs.get_mut::<TestComp3>(entity_id).unwrap();
//         assert_eq!(value.value, TEST_COMP_3_VALUE);
//     }
//
// }