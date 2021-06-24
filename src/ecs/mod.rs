use crate::ecs::superb_ecs::SuperbEcs;

pub mod superb_ecs;
pub mod eid_manager;
pub mod comp_store;
pub mod pending_entity;
pub mod ecs_macros;

pub type GlobalEntityID = usize;
pub type ActiveEcs<T> = SuperbEcs<T>;