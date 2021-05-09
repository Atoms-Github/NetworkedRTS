use serde::{Deserialize, Serialize};
use crate::ecs::System;
use crate::ecs::{ActiveEcs};
use std::any::{Any, TypeId};
use crate::rts::comps::velocity_component::VelocityComp;
use crate::rts::comps::position_comp::PositionComp;
use crate::pub_types::PointFloat;
use std::ops::Add;


#[derive(Clone, Serialize, Deserialize)]
pub struct VeocitylSys {

}

#[typetag::serde]
impl System for VeocitylSys {
    fn run(&self, ecs: &mut ActiveEcs) {
        for entity_id in ecs.query(vec![crate::utils::crack_type_id::<VelocityComp>(), crate::utils::crack_type_id::<PositionComp>()]){
            // ***noice*** /s
            ecs.get_mut::<PositionComp>(entity_id).unwrap().pos.x += ecs.get::<VelocityComp>(entity_id).unwrap().vel.x;
            ecs.get_mut::<PositionComp>(entity_id).unwrap().pos.y += ecs.get::<VelocityComp>(entity_id).unwrap().vel.x;
        }
    }
    fn my_clone(&self) -> Box<dyn System> {
        Box::new(self.clone())
    }
}