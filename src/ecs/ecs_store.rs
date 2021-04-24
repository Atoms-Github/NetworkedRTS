use serde::*;
pub type CompositionID = usize;

pub type VerticalStorage<T> = Vec<Vec<T>>;
//pub type TypeSetSerializable = BTreeSet<u64>;
pub type TypeSet = BTreeSet<TypeIdNum>;



use serde::{Deserialize, Serialize};
use crate::ecs::ecs_manager::*;
use std::collections::{HashMap, BTreeSet, BTreeMap};
use std::any::{Any, TypeId};
use anymap::any::CloneAny;
use std::fmt::Debug;
use std::hash::Hash;
use crate::utils::TypeIdNum;
use anymap::AnyMap;
use crate::ecs::{Ecs, NewEntity, Component};


struct SlicePointer{
    composition_id: CompositionID,
    index_within_composition: usize
}


// #[derive(Clone, Serialize, Deserialize, Debug)]
#[derive(Serialize)]
pub struct EcsStore{
    pub test_vec: Vec<Box<dyn System>>,
    pub vertical_storages: BTreeMap<TypeIdNum, VerticalStorage<Box<dyn Component>>>
}
impl Ecs for EcsStore{
    fn query(&self, entity_id: usize) -> Vec<usize> {
        unimplemented!();
    }

    fn add_entity(&mut self, new_components: NewEntity) {
        unimplemented!()
    }

    fn get_component<T: Component>(&self, entity_id: usize) -> &T {
        unimplemented!()
    }

    fn run_systems(&self, systems: Vec<Box<dyn System>>) {
        unimplemented!();
        for system in systems{
            system.run(&mut self.root_storage);
        }
    }
}







