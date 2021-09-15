
use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::event::MouseButton;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
use std::ops::Mul;
use itertools::Itertools;
use mopa::Any;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct HikerComp { // Walks comp, but includes fliers and sailers too.
    pub waypoints: Vec<PointFloat>,
    pub route_calc_cooldown: i32, // To stutter units performing pathfinding across multiple frames.
    pub speed: f32,
    pub quest_importance: u8,
}
impl HikerComp{
    pub fn set_destination(&mut self, target: PointFloat){
        self.waypoints.clear();
        self.waypoints.push(target);
        self.route_calc_cooldown = 1;
    }
}


pub static HIKER_SYS: System = System{
    run,
    name: "hiker"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    if let Some(arena) = c.find_arena(){
        // Calculating waypoints.
        for (unit_id, hiker, position, order) in
        CompIter3::<HikerComp, PositionComp, OrdersComp>::new(c) {
            if order.state == OrderState::MOVING{
                hiker.route_calc_cooldown -= 1;
                if hiker.route_calc_cooldown == 0{
                    assert_eq!(hiker.waypoints.len(), 1);
                    hiker.waypoints = arena.pathfind(position.pos.clone(), hiker.waypoints.get(0).unwrap().clone());
                    // TODO: Populate waypoints.
                }
            }
        }
        // Move units towards their target.
        for (unit_id, hiker, position, order) in
        CompIter3::<HikerComp, PositionComp, OrdersComp>::new(c) {
            if order.state == OrderState::MOVING{
                let mut made_destination = false;
                if let Some(destination) = hiker.waypoints.first(){
                    let dist_can_move = hiker.speed * crate::netcode::common::time::timekeeping::FRAME_DURATION_MILLIS;
                    if (destination.clone() - &position.pos).magnitude() < dist_can_move{
                        position.pos = destination.clone();
                        made_destination = true;
                    }else{
                        position.pos += (destination.clone() - &position.pos).normalize().mul(dist_can_move);
                    }
                }
                if made_destination{
                    hiker.waypoints.remove(0);
                }

            }
        }
    }

}


