#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(core_intrinsics)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_unsafe)] // TODO2: Investigate the need for this.
#![feature(drain_filter)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(deprecated)] // TODO:



pub mod superb_ecs;
pub mod eid_manager;
pub mod comp_store;
pub mod pending_entity;
pub mod radix_sorting;
pub mod ecs_macros;
pub mod ecs_debug_timer;
pub mod bblocky;
pub mod utils;
pub mod unsafe_utils;

pub type GlobalEntityID = usize;
pub type ZType = u16;

pub use comp_store::*;
pub use superb_ecs::*;
pub use pending_entity::*;
pub use ecs_macros::*;
pub use bblocky::comp_registration::*;

pub use netcode::SimMetadata;
pub use netcode::SimQuality;

