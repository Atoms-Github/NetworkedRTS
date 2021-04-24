#[macro_use]
mod ecs_store;
mod ecs_tests;
mod ecs_manager;
mod systems_lookup;
mod ecs_res;

use anymap::AnyMap;
use crate::ecs::ecs_manager::System;

pub type GlobalEntityID = usize;


#[typetag::serde(tag = "type")]
pub trait Component : mopa::Any{
}
mopa::mopafy!(Component);

pub struct NewEntity{
    pub new_components: AnyMap
}
pub trait Ecs{
    fn query(&self, entity_id: GlobalEntityID) -> Vec<GlobalEntityID>;
    fn add_entity(&mut self, new_components: NewEntity);
    fn get_component<T : Component>(&self, entity_id: GlobalEntityID) -> &T;
    fn run_systems(&self, systems: Vec<Box<dyn System>>);
}

