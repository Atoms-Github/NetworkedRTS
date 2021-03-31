use crate::ecs::ecs_store::*;
use serde::{Deserialize, Serialize};
use crate::ecs::ecs_manager::System;

pub struct VelSystem{
}
impl System for VelSystem {
    fn run(&self, data: &mut EcsStore) {

    }
}