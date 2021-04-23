use crate::ecs::ecs_store::*;
use serde::{Deserialize, Serialize};
use crate::ecs::ecs_manager::System;

#[derive(Serialize, Deserialize)]
pub struct VelSystem{
}

#[typetag::serde]
impl System for VelSystem {
    fn run(&self, data: &mut EcsStore) {

    }
}