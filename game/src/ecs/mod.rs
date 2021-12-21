use crate::ecs::superb_ecs::SuperbEcs;

pub mod superb_ecs;
pub mod eid_manager;
pub mod comp_store;
pub mod pending_entity;
pub mod radix_sorting;
pub mod ecs_macros;
pub mod ecs_debug_timer;
pub mod bblocky;

pub type GlobalEntityID = usize;
pub type ActiveEcs = SuperbEcs;