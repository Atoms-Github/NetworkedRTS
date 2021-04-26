use anymap::AnyMap;

use crate::ecs::holy_ecs::HolyEcs;
pub use crate::ecs::ecs_shared::Component;
pub use crate::ecs::ecs_shared::System;

#[macro_use]
mod holy_ecs;
mod ecs_tests;
pub mod ecs_shared;

pub type GlobalEntityID = usize;
pub type ActiveEcs = HolyEcs;

pub trait Ecs{
    fn query(&self, entity_id: GlobalEntityID) -> Vec<GlobalEntityID>;
    fn add_entity(&mut self, new_components: AnyMap) -> GlobalEntityID;
    fn get_component<T : Component>(&self, entity_id: GlobalEntityID) -> &T;
    fn run_systems(&self, systems: &Vec<Box<dyn System>>);
}

