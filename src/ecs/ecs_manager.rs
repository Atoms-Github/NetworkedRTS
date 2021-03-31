use serde::{Deserialize, Serialize};
use crate::ecs::ecs_store::*;
use std::fmt::Debug;
use std::hash::Hash;
use anymap::any::CloneAny;
use std::any::{TypeId, Any};
use std::collections::HashMap;


pub trait System{
    fn run(&self, data: &mut EcsStore);
}
struct SystemInfo {
    my_system: Box<dyn System>,
    name: String,
}
pub struct SystemsLookup {
    items: Vec<SystemInfo>
}
impl SystemsLookup{
    pub fn new() -> Self{
        SystemsLookup{
            items: vec![]
        }
    }
    pub fn add_sys<T : 'static + System>(&mut self, system: T){
        self.items.push(SystemInfo{
            my_system: Box::new(system),
            name: "".to_string()
        });
    }
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

