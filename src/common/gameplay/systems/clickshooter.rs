use std::collections::{BTreeSet, HashMap};

use serde::{Deserialize, Serialize};

use crate::create_system;
use crate::common::gameplay::ecs::world::*;
use crate::common::gameplay::systems::velocity::VelocityComp;
use crate::common::types::*;
use crate::common::sim_data::input_state::*;
use winit::MouseCursor::Move;
use crate::common::gameplay::systems::position::PositionComp;
use crate::common::gameplay::systems::size::SizeComp;
use crate::common::gameplay::systems::render::RenderComp;


create_system!( clickshooter_system | secret_clickshooter_system
	| my_velocity: VelocityComp, my_clickshooter_comp: ClickShooterComp
	|
	| players_input: &HashMap<PlayerID, InputState>, frame_index: FrameIndex
);

const MOVEMENT_SPEED: f32 = 2.0;

#[derive(Debug,Serialize, Deserialize, Clone, Hash)]
pub struct ClickShooterComp {
    pub owner_id: PlayerID
}

fn clickshooter_system(d: &mut Data, e: Entity, player_inputs: &HashMap<PlayerID, InputState>, frame_index: FrameIndex){
    let owner_id = e.my_clickshooter_comp(d).owner_id;
    let my_inputs = player_inputs.get(&owner_id).unwrap_or_else(||{panic!("Can't find unit owner: {} Simmed: {}", owner_id, frame_index)});
    if my_inputs.mouse_btns_pressed.len() > 0{

        let mut pending_entity_online_player = PendingEntity::new();

        pending_entity_online_player.add_component(PositionComp{ x: 0.0, y: 0.0 });
        pending_entity_online_player.add_component(VelocityComp{ x: 1.0, y: 0.0 });
        pending_entity_online_player.add_component(SizeComp{ x: 25.0, y: 25.0 });
        pending_entity_online_player.add_component(RenderComp{ hue: (99,200,30)});
        d.pending.create_entity(pending_entity_online_player);
    }
}

