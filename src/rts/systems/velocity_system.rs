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
            // let position : &mut PositionComp = ecs.get_mut(entity_id);
            // let velocity : &VelocityComp = ecs.get(entity_id);
            // position.pos.x += velocity.vel.x;
            // position.pos.y += velocity.vel.y;
        }
    }
    fn my_clone(&self) -> Box<dyn System> {
        Box::new(self.clone())
    }
}