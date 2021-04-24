use serde::{Deserialize, Serialize};
use crate::ecs::ecs_store::*;
use std::fmt::Debug;
use std::hash::Hash;
use anymap::any::CloneAny;
use std::any::{TypeId, Any};
use std::collections::HashMap;
use crate::ecs::systems_lookup::SystemsLookup;

#[typetag::serde(tag = "type")]
pub trait System{
    fn run(&self, data: &mut EcsStore);
}




// #[derive(Clone, Serialize, Deserialize)]
pub struct EcsManager {
    root_storage: EcsStore,
    systems: Vec<Box<dyn System>>,
}

impl EcsManager {
    pub fn new() -> Self{
        Self{
            root_storage: EcsStore::new(),
            systems: vec![]
        }
    }
    pub fn sim(&mut self, systems_lookup: &SystemsLookup){

    }
}

