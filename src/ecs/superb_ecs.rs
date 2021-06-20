

use serde::*;
use anymap::AnyMap;
use crate::ecs::comp_store::*;


// TODO: Implement ser and de manually.
pub struct SuperbEcs<R :Clone>{
    systems: Vec<System<R>>,
    comp_storage: CompStorage,
}
impl<R : Clone> SuperbEcs<R>{
    pub fn new(systems: Vec<System<R>>) -> Self{
        Self{
            systems: vec![],
            comp_storage: Default::default(),
        }
    }
    pub fn set_systems(&mut self, systems: Vec<System<R>>){
        self.systems = systems;
    }
    pub fn sim_systems(&mut self, resources: R){
        for system in &self.systems{
            (system.run)(&resources, &mut self.comp_storage);
        }
    }
}

#[derive(Clone)]
pub struct System<R : Clone>{
    run: fn(&R, &mut CompStorage /* Could add read only version here. */),
}



