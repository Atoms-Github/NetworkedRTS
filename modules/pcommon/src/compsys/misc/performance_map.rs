use crate::*;
use winit::event::MouseButton;
use winit::event::VirtualKeyCode;
use std::ops::Mul;
use nalgebra::{distance, distance_squared};


pub static PERFORMANCE_MAP: System = System{
    run,
    name: "orders"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    // if let Some(arena) = c.find_arena(){
    //     arena.clear_performance_map();
    //     for (unit_id, position, owned, life)
    //     in CompIter3::<PositionComp, OwnedComp>::new(c) {
    //         arena.register_performance_map_entity(unit_id, &position.pos)
    //     }
    // } TODO!
}












