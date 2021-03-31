use crate::ecs::ecs_store::*;
use crate::ecs::ecs_manager::{System, EcsStore};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Hash)]
pub struct VelocitySystem;

impl<'a> System for VelocitySystem {
    fn run(&mut self, data: &mut EcsStore) {
        println!("Running!");
    }
}

#[derive(Clone, Serialize, Deserialize, Hash)]
pub struct VelocitySystemTwo;

impl System for VelocitySystemTwo {
    fn run(&mut self, data: &mut EcsStore) {
        println!("Running!Two");
    }
}
