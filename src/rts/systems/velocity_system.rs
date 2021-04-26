use serde::{Deserialize, Serialize};
use crate::ecs::System;
use crate::ecs::{Ecs, ActiveEcs};


pub fn sys_vel(){

}
#[derive(Clone, Serialize, Deserialize)]
pub struct VelSystem{
}

#[typetag::serde]
impl System for VelSystem {
    fn run(&self, ecs: &ActiveEcs) {
        unimplemented!()
    }

    fn my_clone(&self) -> Box<dyn System> {
        Box::new(self.clone())
    }
}