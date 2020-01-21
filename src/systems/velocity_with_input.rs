use std::collections::{BTreeSet};

use serde::{Deserialize, Serialize};

use crate::create_system;
use crate::ecs::world::*;
use crate::network::networking_structs::*;
use crate::systems::velocity::VelocityComp;

//use crate::inputs::input_structs::*;

create_system!( velocity_with_inputs_system | secret_velocity_with_inputs_system
	| my_velocity: VelocityComp, my_velocity_with_input_comp: VelocityWithInputComp
	|
	| players_input: &InfoForSim
);


const MOVEMENT_SPEED: f32 = 2.0;


fn velocity_with_inputs_system(d: &mut Data, e: Entity, player_inputs: &InfoForSim) {
//    let owner_id = e.my_velocity_with_input_comp(d).owner_id; TODO1: Implement
//    let my_inputs = player_inputs.inputs.get(&owner_id).expect("Can't find inputs for unit owner.");
//
//    let (directional_x, directional_y) = my_inputs.get_directional();
////    println!("X: {} Y: {}", directional_x, directional_y);
//    e.my_velocity(d).x = MOVEMENT_SPEED * directional_x;
//    e.my_velocity(d).y = MOVEMENT_SPEED * -directional_y;
}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct VelocityWithInputComp {
    pub owner_id: usize
}