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
use std::collections::{HashMap, BTreeSet, BTreeMap};
use std::any::{Any, TypeId};
use anymap::any::CloneAny;
use std::fmt::Debug;
use std::hash::Hash;
use crate::utils::TypeIdNum;


#[typetag::serde(tag = "type")]
pub trait Component : mopa::Any{
}

mopa::mopafy!(Component);





// #[derive(Clone, Serialize, Deserialize, Debug)]
#[derive(Serialize)]
pub struct EcsStore{
    pub test_vec: Vec<Box<dyn System>>,
    pub vertical_storages: BTreeMap<TypeIdNum, VerticalStorage<Box<dyn Component>>>
}

impl EcsStore{
    pub fn new() -> Self{
        Self{
            vertical_storages: Default::default(),
            test_vec: vec![]
        }
    }
    pub fn get_component<T>(&self, entity_id: GlobalEntityID){
    }
    pub fn init_component_type<T>(&mut self){
        // self.vertical_storages.insert(T::type_id(), vec![]);
    }
}







