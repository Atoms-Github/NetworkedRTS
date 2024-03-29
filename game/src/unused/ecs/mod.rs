use anymap::AnyMap;

pub use crate::ecs::ecs_shared::System;
use crate::ecs::holy_ecs::HolyEcs;
use crate::ecs::systems_man::SystemsMan;
use std::any::TypeId;
use ecs_shared::SerdeObject;
use crate::ecs::ecs_shared::Component;
use crate::ecs::my_anymap::SerdeAnyMap;


#[macro_use]
mod ecs_tests;
pub mod my_anymap;
pub mod unmoving_vec;
pub mod macro_version;
pub mod holy_ecs;
pub mod ecs_shared;
pub mod systems_man;
pub mod liquid_garbage;


pub type GlobalEntityID = usize;
pub type ActiveEcs = HolyEcs;

// pub trait Ecs{
//     fn add_entity(&mut self, new_components: SerdeAnyMap) -> GlobalEntityID;
//     fn query(&self, types: Vec<TypeId>) -> Vec<GlobalEntityID>;
//     fn get<T : SerdeObject>(&self, entity_id: GlobalEntityID) -> &T;
//     fn get_mut<T: SerdeObject>(&mut self, entity_id: usize) -> &mut T;
//     fn run_systems(&mut self, systems: &SystemsMan);
// }
//
