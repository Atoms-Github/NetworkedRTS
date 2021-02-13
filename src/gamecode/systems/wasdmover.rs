use std::collections::{BTreeSet, HashMap};

use serde::{Deserialize, Serialize};

use crate::netcode::create_system;
use crate::gamecode::ecs::world::*;
use crate::gamecode::systems::velocity::VelocityComp;
use crate::netcode::common::types::*;
use crate::netcode::common::sim_data::input_state::*;
use crate::gamecode::systems::position::PositionComp;
use crate::gamecode::systems::size::SizeComp;
use crate::gamecode::systems::render::RenderComp;

//use crate::netcode::sync::input_structs::*;

create_system!( wasdmover_system | secret_wasdmover_system
	| my_velocity: VelocityComp, my_wasdmover_comp: WasdMoverComp
	|
	| players_input: &HashMap<PlayerID, InputState>, frame_index: FrameIndex
);
const MOVEMENT_SPEED: f32 = 6.0;

#[derive(Debug,Serialize, Deserialize, Clone, Hash)]
pub struct WasdMoverComp {
    pub owner_id: PlayerID
}

fn wasdmover_system(d: &mut Data, e: Entity, player_inputs: &HashMap<PlayerID, InputState>, frame_index: FrameIndex){


    let owner_id = e.my_wasdmover_comp(d).owner_id;
    let my_inputs = player_inputs.get(&owner_id).unwrap_or_else(||{panic!("Can't find unit owner: {} Simmed: {}", owner_id, frame_index)});

    let (directional_x, directional_y) = my_inputs.get_directional();

    let mut my_speed = MOVEMENT_SPEED;

    if my_inputs.mouse_btns_pressed.len() > 0{
        //my_speed *= 0.5;
    }

    e.my_velocity(d).x = my_speed * directional_x;
    e.my_velocity(d).y = my_speed * -directional_y;


}

