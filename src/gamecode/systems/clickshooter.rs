use std::collections::{BTreeSet, HashMap};

use serde::{Deserialize, Serialize};


use crate::gamecode::ecs::world::*;
use crate::gamecode::systems::velocity::VelocityComp;
use crate::gamecode::systems::position::PositionComp;
use crate::gamecode::systems::size::SizeComp;
use crate::gamecode::systems::render::RenderComp;
use std::ops::{Sub, Mul};
use std::hash::Hasher;
use std::hash::*;
use crate::gamecode::ecs::*;
use crate::pub_types::*;
use crate::netcode::{InputState, PlayerInputs};


create_system!( clickshooter_system | secret_clickshooter_system
	| my_velocity: VelocityComp, my_clickshooter_comp: ClickShooterComp, my_position: PositionComp
	|
	| players_input: &HashMap<PlayerID, InputState>, frame_index: FrameIndex
);

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct ClickShooterComp {
    pub owner_id: PlayerID,
    pub cooldown: f32
}
impl Hash for ClickShooterComp{
    fn hash<H: Hasher>(&self, state: &mut H) { // Can fix with fixed and/or cordick first.
        self.owner_id.to_be_bytes().hash(state);
        self.cooldown.to_be_bytes().hash(state);
    }
}

fn clickshooter_system(d: &mut Data, e: Entity, player_inputs: &PlayerInputs, frame_index: FrameIndex){
    let owner_id = e.my_clickshooter_comp(d).owner_id;
    let my_inputs = player_inputs.get(&owner_id).unwrap_or_else(||{panic!("Can't find unit owner: {} Simmed: {}", owner_id, frame_index)});
    if my_inputs.mouse_btns_pressed.len() > 0{

        if e.my_clickshooter_comp(d).cooldown <= 0.0{
            let target = my_inputs.mouse_loc.coords.clone();
            let current_location = PointFloat::new(e.my_position(d).x, e.my_position(d).y).coords;
            let velocity_vec = PointFloat::new(target.x - current_location.x, target.y - current_location.y).coords.normalize().mul(4.0);
            let velocity = VelocityComp{ x: velocity_vec.x, y: velocity_vec.y };

            let mut pending_entity_bullet = PendingEntity::new();
            pending_entity_bullet.add_component(e.my_position(d).clone());
            pending_entity_bullet.add_component(velocity);
            pending_entity_bullet.add_component(SizeComp{ x: 25.0, y: 25.0 });
            pending_entity_bullet.add_component(RenderComp{ hue: (99,200,30)});
            d.pending.create_entity(pending_entity_bullet);

            e.my_clickshooter_comp(d).cooldown = 20.0;
        }
    }
    e.my_clickshooter_comp(d).cooldown -= 1.0;
}

