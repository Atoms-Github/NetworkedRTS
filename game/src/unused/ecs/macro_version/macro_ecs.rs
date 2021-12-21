use serde::*;
use anymap::AnyMap;
use crate::ecs::macro_version::macro_mess::MacroMess;
use crate::ecs::macro_version::generic_version::EntityManager;



// TODO: Implement ser and de manually.
pub struct MacroEcs<R :Clone>{
    systems: Vec<System>,
    eman: EntityManager,
}
impl<R : Clone> MacroEcs{
    pub fn new(systems: Vec<System>) -> Self{
        Self{
            systems: vec![],
            eman: EntityManager::new(),
        }
    }
    pub fn set_systems(&mut self, systems: Vec<System>){
        self.systems = systems;
    }
    pub fn sim_systems(&mut self, resources: R){
        for system in &self.systems{
            (system.run)(&resources, &mut self.eman);
        }
    }
}

#[derive(Clone)]
pub struct System<R : Clone>{
    run: fn(&R, &mut EntityManager /* Could add read only version here. */),
}

