use serde::{Deserialize, Serialize};
use crate::ecs::System;
use crate::ecs::{ActiveEcs};
use std::any::{Any, TypeId};
use crate::rts::comps::velocity_component::VelocityComp;
use crate::rts::comps::position_comp::PositionComp;
use crate::pub_types::PointFloat;
use std::ops::Add;
use crate::rts::comps::owner_comp::OwnedComp;
use crate::rts::comps::player_comp::PlayerComp;
use crate::rts::comps::velocity_with_inputs_comp::VelocityWithInputsComp;


#[derive(Clone, Serialize, Deserialize)]
pub struct VelocityWithInputsSys {
}

#[typetag::serde]
impl System for VelocityWithInputsSys {
    fn run(&self, ecs: &mut ActiveEcs) {
        for entity_id in ecs.query(vec![crate::utils::crack_type_id::<VelocityComp>(), crate::utils::crack_type_id::<VelocityWithInputsComp>(), crate::utils::crack_type_id::<OwnedComp>()]){
            let owner_id = ecs.get::<OwnedComp>(entity_id).unwrap().owner;
            let my_inputs = ecs.get::<PlayerComp>(owner_id).unwrap().inputs.clone();

            let (directional_x, directional_y) = my_inputs.get_directional();

            let mut my_speed = ecs.get::<VelocityWithInputsComp>(entity_id).unwrap().speed;

            if my_inputs.mouse_btns_pressed.len() > 0{
                my_speed *= 2.0;
            }
            let velocity = ecs.get_mut::<VelocityComp>(entity_id).unwrap();
            velocity.vel.x = my_speed * directional_x;
            velocity.vel.y = my_speed * -directional_y;
        }
    }
    fn my_clone(&self) -> Box<dyn System> {
        Box::new(self.clone())
    }
}