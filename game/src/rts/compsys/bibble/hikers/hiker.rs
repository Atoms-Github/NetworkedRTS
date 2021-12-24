
use crate::rts::compsys::jigsaw::jigsaw_game_state::*;
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
    state: HikerPathState,
    pub speed: f32,
    pub quest_importance: u8,
}
impl HikerComp{
    pub fn remove_waypoint(&mut self){
        let mut set_to_stationary = false;
        match &mut self.state{
            HikerPathState::PENDING_PATHFIND { .. } => {
                set_to_stationary = true;
            }
            HikerPathState::GOT_PATH { path } => {
                path.remove(0);
                if path.len() == 0{
                    set_to_stationary = true;
                }
            }
            HikerPathState::STATIONARY => {}
        };
        if set_to_stationary{
            self.state = HikerPathState::STATIONARY;
        }
    }
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
enum HikerPathState{
    PENDING_PATHFIND{route_calc_cooldown: i32, destination: PointFloat},
    GOT_PATH{path: Vec<PointFloat>},
    STATIONARY,
}
impl HikerComp{
    pub fn new(speed: f32) -> Self{
        Self{
            state: HikerPathState::STATIONARY,
            speed,
            quest_importance: 0
        }
    }
    pub fn get_destination(&self) -> Option<PointFloat>{
        match &self.state{
            HikerPathState::PENDING_PATHFIND { route_calc_cooldown, destination } => {Some(destination.clone() as PointFloat)}
            HikerPathState::GOT_PATH { path } => {path.last().cloned()}
            HikerPathState::STATIONARY => {None}
        }
    }
    pub fn set_destination(&mut self, target: PointFloat, quest_importance: u8){
        self.state = HikerPathState::PENDING_PATHFIND { route_calc_cooldown: 1, destination: target };
        self.quest_importance = quest_importance;
    }
}


pub fn hiker_sys<C>() -> System<C>{
    System{
        run,
        name: "hiker"
    }
}
fn run<C>(c: &mut CompStorage<C>, ent_changes: &mut EntStructureChanges<C>, meta: &SimMetadata){
    if let Some(arena) = c.find_arena(){
        // Calculating waypoints.
        for (unit_id, hiker, position, order) in
        CompIter3::<HikerComp, PositionComp, OrdersComp>::new(c) {
            if let HikerPathState::PENDING_PATHFIND { route_calc_cooldown, destination } = &mut hiker.state{
                *route_calc_cooldown -= 1;
            }
            if let HikerPathState::PENDING_PATHFIND { route_calc_cooldown, destination } = hiker.state.clone(){
                if route_calc_cooldown == 0 && meta.quality == SimQuality::DETERMA{ // On head, just never resolve.
                    let path = arena.pathfind(position.pos.clone(), destination.clone(), 10.0);
                    hiker.state = HikerPathState::GOT_PATH { path }
                }
            }
        }
        // Move units towards their target.
        for (unit_id, hiker, position, order) in
        CompIter3::<HikerComp, PositionComp, OrdersComp>::new(c) {
            let mut straight_towards : Option<PointFloat> = match &hiker.state{
                HikerPathState::PENDING_PATHFIND {  destination , ..} => {
                    let point : PointFloat = destination.clone();
                    Some(point)
                }
                HikerPathState::GOT_PATH { path } => {Some(path.get(0).unwrap().clone() as PointFloat)}
                HikerPathState::STATIONARY => {None}
            };
            if let Some(target) = straight_towards{
                let dist_can_move = hiker.speed * meta.delta;
                if (target.clone() - &position.pos).magnitude() < dist_can_move{
                    // We've made it to the target.
                    position.pos = target;
                    hiker.remove_waypoint();
                }else{
                    position.pos += (target.clone() - &position.pos).normalize().mul(dist_can_move);
                }
            }
        }
    }
}


