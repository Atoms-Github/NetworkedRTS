use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};
use std::any::TypeId;
use crate::ecs::world::*;
use crate::systems::velocity::VelocityComp;
use crate::create_system;
use crate::network::networking_structs::*;

//use crate::inputs::input_structs::*;

create_system!( velocityWithInput_system | secret_velocityWithInput_system
	| my_velocity: VelocityComp, my_velocityWithInput: velocityWithInputComp
	|
	| players_input: &InputsFrame
);

fn velocityWithInput_system(d: &mut Data, e: Entity, controllers: &InputsFrame) {
    let controller = controllers.get(e.my_velocityWithInput(d).owner_id).unwrap();
    e.my_velocity(d).x = 1.0 * controller.input_state.directional.x as f32;
    e.my_velocity(d).y = 1.0 * -controller.input_state.directional.y as f32;

}

#[derive(Debug, Clone)]
pub struct velocityWithInputComp {
    pub owner_id: usize
}