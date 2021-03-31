use serde::{Deserialize, Serialize};
use crate::ecs::ecs_store::*;
use std::fmt::Debug;
use std::hash::Hash;
use anymap::any::CloneAny;
use std::any::{TypeId, Any};
use std::collections::HashMap;
use crate::ecs::systems_lookup::SystemsLookup;


pub trait System{
    fn run(&self, data: &mut EcsStore);
}




#[derive(Clone, Serialize, Deserialize, Hash)]
pub struct EcsManager {
    root_storage: EcsStore,
}

impl EcsManager {
    pub fn new() -> Self{
        Self{
            root_storage: EcsStore::new()
        }
    }
    pub fn sim(&mut self, systems_lookup: &SystemsLookup){
        for system in &systems_lookup.items{
            system.my_system.run(&mut self.root_storage);
        }
    }
}

