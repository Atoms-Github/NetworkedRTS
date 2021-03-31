use serde::*;
pub type PogTypeId = u64;
pub type CompositionID = usize;
pub type GlobalEntityID = usize;
pub type VerticalStorage<T> = Vec<Vec<T>>;
//pub type TypeSetSerializable = BTreeSet<u64>;
//pub type TypeSetTypes = BTreeSet<TypeId>;
pub type TypeSet = BTreeSet<PogTypeId>;



use serde::{Deserialize, Serialize};
use crate::ecs::ecs_manager::*;
use std::collections::{HashMap, BTreeSet};
use std::any::{Any, TypeId};
use anymap::any::CloneAny;
use std::fmt::Debug;
use std::hash::Hash;




#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct EcsStore{
    // pub vertical_storages: HashMap<TypeId, VerticalStorage<Box<dyn CloneAny>>> // TODO: Test with just a single AnyMap thing. Does it work generics?

}

impl EcsStore{
    pub fn new() -> Self{
        Self{
            // vertical_storages: Default::default()
        }
    }
    pub fn get_component<T>(&self, entity_id: GlobalEntityID){

    }
    pub fn init_component_type<T>(&mut self){
        // self.vertical_storages.insert(T::type_id(), vec![]);
    }
}







