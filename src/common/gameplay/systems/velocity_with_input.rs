use std::collections::{BTreeSet, HashMap};

use serde::{Deserialize, Serialize};

use crate::create_system;
use crate::common::gameplay::ecs::world::*;
use crate::common::gameplay::systems::velocity::VelocityComp;
use crate::common::types::*;
use crate::common::sim_data::input_state::*;

//use crate::sync::input_structs::*;

create_system!( velocity_with_inputs_system | secret_velocity_with_inputs_system
	| my_velocity: VelocityComp, my_velocity_with_input_comp: VelocityWithInputComp
	|
	| players_input: &HashMap<PlayerID, InputState>, frame_index: FrameIndex
);
const MOVEMENT_SPEED: f32 = 10.0;

fn velocity_with_inputs_system(d: &mut Data, e: Entity, player_inputs: &HashMap<PlayerID, InputState>, frame_index: FrameIndex){

    let owner_id = e.my_velocity_with_input_comp(d).owner_id;
    let my_inputs = player_inputs.get(&owner_id).unwrap_or_else(||{panic!("Can't find unit owner: {} Simmed: {}", owner_id, frame_index)});

    let (directional_x, directional_y) = my_inputs.get_directional();
//    println!("X: {} Y: {}", directional_x, directional_y);
    e.my_velocity(d).x = MOVEMENT_SPEED * directional_x;
    e.my_velocity(d).y = MOVEMENT_SPEED * -directional_y;
}

#[derive(Debug,Serialize, Deserialize, Clone, Hash)]
pub struct VelocityWithInputComp {
    pub owner_id: PlayerID
}