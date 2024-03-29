use std::collections::{BTreeMap, BTreeSet};
use crate::utils::TypeIdNum;
use serde::*;
use crate::ecs::eid_manager::{GlorifiedHashMap, GlobalEntityID};
use anymap::AnyMap;
use crate::ecs::pending_entity::PendingEntity;
use mopa::Any;
use crate::ecs::bblocky::*;
use crate::ecs::bblocky::super_any::SuperAny;
use crate::ecs::bblocky::super_vec::SuperVec;
use crate::pub_types::ZType;

pub type SingleComp<C> = SuperAny<C>;
pub type MyBlock<C> = SuperVec<C>;
pub type Column<C> = Vec<MyBlock<C>>;

pub type CompositionID = usize;
pub type GenerationNum = usize;
pub type InternalIndex = usize;
pub type TypesSet = BTreeSet<TypeIdNum>;

#[derive(Clone, Serialize, Default, Deserialize, Debug, Hash, PartialEq)]
pub struct InternalEntity {
    pub comp_types: TypesSet,
    pub global_id: GlobalEntityID,
    composition_id: CompositionID,
    internal_index: InternalIndex,
}

#[derive(Clone, Default, Serialize, Deserialize, Hash, Debug)]
pub struct CompStorage<C> {
    columns:BTreeMap<TypeIdNum, Column<C>>,
    internal_entities: Box<GlorifiedHashMap>, // Box to avoid stack overflow.
    composition_ids: BTreeMap<TypesSet, CompositionID>,
    next_composition_id: CompositionID,
    global_ids_as_comps: Vec<Vec<GlobalEntityID>>, // (Pretty much a column<GlobalEntityID>)
}
pub struct DeleteResult{

}
impl<C> CompStorage<C>{
    // TODO: Swap get and get unwrap, and call them get and get_maybe.
    pub fn get_unwrap<T : 'static>(&self, entity_id: GlobalEntityID) -> &T{
        self.get::<T>(entity_id).unwrap()
    }
    pub fn get_mut_unwrap<T : 'static>(&self, entity_id: GlobalEntityID) -> &mut T{
        self.get_mut::<T>(entity_id).unwrap()
    }
    pub fn get<T : 'static>(&self, entity_id: GlobalEntityID) -> Option<&T>{
        // Pog. Can be pure 0 processing time function if we use get_unsafe().
        let internal = self.internal_entities.get(entity_id)?;
        let column = self.get_column::<T>()?;
        let block = column.get(internal.composition_id)?;
        return block.get(internal.internal_index);
    }
    pub fn ent_alive(&self, entity_id: GlobalEntityID) -> bool{
        return self.internal_entities.get(entity_id).is_some();
    }
    pub fn get_mut<T : 'static>(&/*Non-mut. Unsafe loh.*/self, entity_id: GlobalEntityID) -> Option<&mut T>{
        return self.get::<T>(entity_id).map(|unmut|{unsafe{crate::unsafe_utils::very_bad_function(unmut)}});
    }
    pub fn delete_entity(&mut self, entity_id: GlobalEntityID) -> bool{
        // What we want to do:
        // Find all the components that the entity has.
        // Find the entity's composition ID.
        // Delete a horizontal slice in the correct place, and move the entity at the bottom of that into that space.
        // Find the internal entity block of the moved entity, and update it.
        // Remember to also delete in global_ids_as_comps.
        let deleting_internal = self.internal_entities.get(entity_id);
        if deleting_internal.is_none(){
            return false;
        }
        let deleting_internal = deleting_internal.unwrap();
        let composition_id = deleting_internal.composition_id;
        let deleting_index: InternalIndex = deleting_internal.internal_index;



        // Do the deleting.
        {
            for my_type in &deleting_internal.comp_types{
                self.columns.get_mut(&my_type).unwrap().get_mut(composition_id).unwrap().swap_remove(deleting_index);
            }
            self.global_ids_as_comps.get_mut(composition_id).unwrap().swap_remove(deleting_index);
            self.internal_entities.delete(entity_id);
        }
        // Fix the displaced entity's pointer. If lens are same, then took off end.
        let did_swap = deleting_index != self.global_ids_as_comps.get(composition_id).unwrap().len();
        if did_swap {
            let displaced_global_id = self.global_ids_as_comps.get(composition_id).unwrap().get(deleting_index).unwrap();
            let displaced_internal = self.internal_entities.get_mut(*displaced_global_id).unwrap();
            displaced_internal.internal_index = deleting_index;
        }
        return true;




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
    pub fn create_entity(&mut self, pending_entity: PendingEntity<C>) -> InternalIndex{
        let types_set = pending_entity.hash_types();
        let composition_id = self.get_composition_id(types_set.clone());

        let mut internal_index : Option<InternalIndex> = None;
        for (type_id, bytes) in pending_entity.iter(){
            let block = self.get_block_or_make(*type_id, composition_id);
            let new_internal_index = block.len();
            block.push_super_any(bytes.clone()); // TODO: Why need to clone?

            // Confirm that the 'end' of each block is the same place.
            if let Some(existing_internal_index) = internal_index{
                assert_eq!(existing_internal_index, new_internal_index, "ECS internal indices for new entity were different!");
            }else{
                internal_index = Some(new_internal_index);
            }
        }

        let internal_entity = InternalEntity{
            comp_types: types_set,
            global_id: 99999998,
            composition_id,
            internal_index: internal_index.expect("New entity had 0 components! This is disallowed."),
        };
        let global_entity_id = self.internal_entities.create_entity(internal_entity);

        self.save_global_entity_id(composition_id, global_entity_id);


        return global_entity_id;
    }
    pub fn get_entity_count(&self) -> usize{
        self.query(vec![]).len()
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
    pub fn query_sorted(&self, must_include: Vec<TypeIdNum>,
                        sort_by: fn(&CompStorage<C>, GlobalEntityID) -> ZType) -> Vec<GlobalEntityID>{
        let mut unsorted = self.query(must_include);
        return super::radix_sorting::go(self, unsorted, sort_by)
    }
    fn save_global_entity_id(&mut self, composition_id: CompositionID, global_id: GlobalEntityID){
        for new_block_index in self.global_ids_as_comps.len()..(composition_id + 1){
            self.global_ids_as_comps.push(vec![]);
        }
        self.global_ids_as_comps.get_mut(composition_id).unwrap().push(global_id);
    }
    fn get_block_or_make(&mut self, type_id: TypeIdNum, composition_id: CompositionID) -> &mut MyBlock<C>{
        let column = self.get_column_mut_or_make_key(type_id);
        for new_block_index in column.len()..(composition_id + 1){
            column.push(SuperVec::new(type_id));
        }
        return column.get_mut(composition_id).unwrap();
    }
    fn get_column<T : 'static>(&self) -> Option<&Column<C>>{
        self.columns.get(&crate::utils::gett::<T>())
    }
    fn get_column_mut_or_make<T : 'static>(&mut self) -> &mut Column<C>{
        let key = crate::utils::gett::<T>();
        return self.get_column_mut_or_make_key(key);
    }
    fn get_column_mut_or_make_key(&mut self, type_id: TypeIdNum) -> &mut Column<C>{
        if self.columns.get(&type_id).is_none(){
            self.columns.insert(type_id, vec![]);
        }
        return self.columns.get_mut(&type_id).unwrap();
    }

}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TestComp1 {
    value: usize,
    value2: f32,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TestComp0 {
    value: usize,
}
#[cfg(test)]
pub mod ecs_tests {
    use super::*;



    #[test]
    fn test_delete_123() {
        let mut ecs = CompStorage::default();
        let mut pending_entity = PendingEntity::new();
        pending_entity.add_comp(TestComp0{ value: 0});
        let id0 = ecs.create_entity(pending_entity);

        let mut pending_entity = PendingEntity::new();
        pending_entity.add_comp(TestComp0{ value: 1 });
        let id1 = ecs.create_entity(pending_entity);

        let mut pending_entity = PendingEntity::new();
        pending_entity.add_comp(TestComp0{ value: 2});
        let id2 = ecs.create_entity(pending_entity);

        assert_eq!(ecs.get::<TestComp0>(id1).unwrap().value, 1);
        ecs.delete_entity(id0);

        assert_eq!(ecs.get::<TestComp0>(id1).unwrap().value, 1);
        assert_eq!(ecs.get::<TestComp0>(id2).unwrap().value, 2);

        ecs.delete_entity(id1);

        assert_eq!(ecs.get::<TestComp0>(id2).unwrap().value, 2);

        ecs.delete_entity(id2);


        assert!(ecs.get::<TestComp0>(id2).is_none());
    }
    #[test]
    fn test_delete_21() {
        let mut ecs = CompStorage::default();
        let mut pending_entity = PendingEntity::new();
        pending_entity.add_comp(TestComp1{ value: 1, value2: 1.1 });
        let id0 = ecs.create_entity(pending_entity);

        let mut pending_entity = PendingEntity::new();
        pending_entity.add_comp(TestComp1{ value: 2, value2: 2.2 });
        let id1 = ecs.create_entity(pending_entity);

        ecs.delete_entity(id1);

        assert_eq!(ecs.get::<TestComp1>(id0).unwrap().value, 1);

        ecs.delete_entity(id0);

        assert!(ecs.get::<TestComp1>(id1).is_none());
    }
    #[test]
    fn test_delete_then_query() {
        let mut ecs = CompStorage::default();
        let mut pending_entity = PendingEntity::new();
        pending_entity.add_comp(TestComp1{ value: 1, value2: 1.1 });
        let id0 = ecs.create_entity(pending_entity);

        ecs.delete_entity(id0);

        for ent_id in ecs.query(vec![
            crate::utils::gett::<TestComp1>()
        ]){
            if ecs.get::<TestComp1>(ent_id).is_none(){
                let woah = 2;
            }
        }
        for ent_id in ecs.query(vec![
            crate::utils::gett::<TestComp1>()
        ]){
        }





        for ent_id in ecs.query(vec![
            crate::utils::gett::<TestComp1>()

        ]){
            let crash = ecs.get::<TestComp1>(ent_id).unwrap();
        }


    }

    // impl Component for TestComp1 {}
    // impl SerdeObject for TestComp1 {
    //     fn my_clone(&self) -> Box<dyn SerdeObject> {
    //         Box::new(self.clone())
    //     }
    //     fn my_ser(&self) -> Vec<u8> {
    //         return bincode::serialize(self).unwrap();
    //     }
    // }
    // #[derive(Clone, Serialize, Deserialize)]
    // pub struct TestComp2 {
    //     value: f32
    // }
    // impl Component for TestComp2 {}
    // impl SerdeObject for TestComp2 {
    //     fn my_clone(&self) -> Box<dyn SerdeObject> {
    //         Box::new(self.clone())
    //     }
    //     fn my_ser(&self) -> Vec<u8> {
    //         return bincode::serialize(self).unwrap();
    //     }
    // }
    // #[derive(Clone, Serialize, Deserialize)]
    // pub struct TestComp3 {
    //     value: usize
    // }
    // impl Component for TestComp3 {}
    // impl SerdeObject for TestComp3 {
    //     fn my_clone(&self) -> Box<dyn SerdeObject> {
    //         Box::new(self.clone())
    //     }
    //     fn my_ser(&self) -> Vec<u8> {
    //         return bincode::serialize(self).unwrap();
    //     }
    // }
    // fn new_entity() -> (HolyEcs, GlobalEntityID){
    //     let mut ecs = HolyEcs::new();
    //     let mut new_entity_comps = SerdeAnyMap::new();
    //     new_entity_comps.insert(TestComp3{value : TEST_COMP_3_VALUE});
    //     new_entity_comps.insert(TestComp2{value : 3.2});
    //
    //     let new_entity_id = ecs.new_entity(new_entity_comps);
    //     assert_eq!(new_entity_id, 0);
    //     return (ecs, new_entity_id);
    // }
    // #[test]
    // fn ecs_new_entity() {
    //     new_entity();
    // }
    // #[test]
    // fn ecs_query_positive() {
    //     let (mut ecs, entity_id) = new_entity();
    //     let query_results = ecs.query(vec![crate::utils::get_type_id::<TestComp2>()]);
    //     assert_eq!(1, query_results.len());
    //     assert_eq!(entity_id, *query_results.get(0).unwrap());
    // }
    // #[test]
    // fn ecs_query_negative() {
    //     let (mut ecs, entity_id) = new_entity();
    //     let query_results = ecs.query(vec![crate::utils::get_type_id::<TestComp1>(), crate::utils::get_type_id::<TestComp3>()]);
    //     assert_eq!(0, query_results.len());
    // }
    // #[test]
    // fn ecs_get_comp() {
    //     let (mut ecs, entity_id) = new_entity();
    //     let value = ecs.get_mut::<TestComp3>(entity_id).unwrap();
    //     assert_eq!(value.value, TEST_COMP_3_VALUE);
    // }

}