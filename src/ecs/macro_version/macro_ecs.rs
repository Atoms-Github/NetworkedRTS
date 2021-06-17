use serde::*;
use anymap::AnyMap;
use crate::ecs::macro_version::macro_mess::MacroMess;

#[derive(Serialize, Deserialize, PartialEq, Hash, Default)]
pub struct LifeC {

}

// TODO: Implement ser and de manually.
pub struct MacroEcs<R :Clone>{
    systems: Vec<System<R>>,
    macromess: MacroMess
}
impl<R : Clone> MacroEcs<R>{
    pub fn new(systems: Vec<System<R>>) -> Self{
        Self{
            systems: vec![],
            macromess: MacroMess::new(),
        }
    }
    pub fn set_systems(&mut self, systems: Vec<System<R>>){
        self.systems = systems;
    }
    pub fn sim_systems(&mut self, resources: R){
        for system in &self.systems{
            (system.run)(&resources, &mut self.macromess);
        }
    }
}

#[derive(Clone)]
pub struct System<R : Clone>{
    run: fn(&R, &mut MacroMess /* Could add read only macromess here. */),
}

#[derive(Serialize, Deserialize, Hash, Default)]
pub struct EStorage<T>{
    items: Vec<Vec<T>>
}
impl<T> EStorage<T>{
    pub fn new() -> Self{
        Self{
            items: Vec::new()
        }
    }
}