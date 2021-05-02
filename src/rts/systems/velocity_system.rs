use serde::{Deserialize, Serialize};
use crate::ecs::System;
use crate::ecs::{Ecs, ActiveEcs};
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
        for entity_id in ecs.query(vec![TypeId::of::<VelocityComp>(), TypeId::of::<PositionComp>()]){
            // ***noice*** /s
            ecs.get_mut::<PositionComp>(entity_id).pos.x += ecs.get::<VelocityComp>(entity_id).vel.x;
            ecs.get_mut::<PositionComp>(entity_id).pos.y += ecs.get::<VelocityComp>(entity_id).vel.x;
        }
    }
    fn my_clone(&self) -> Box<dyn System> {
        Box::new(self.clone())
    }
}