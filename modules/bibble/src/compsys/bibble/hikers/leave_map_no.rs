use crate::*;
use crate::bibble::*;
use std::ops::Mul;
use nalgebra::{distance, distance_squared};


pub static NO_LEAVE_MAP: System = System{
    run,
    name: "orders"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    if let Some(arena) = c.find_arena(){
        for (unit_id, position)
        in CompIter1::<PositionComp>::new(c) {
            position.pos.x = position.pos.x.clamp(arena.get_left() as f32, arena.get_right() as f32);
            position.pos.y = position.pos.y.clamp(arena.get_top() as f32, arena.get_bottom() as f32);
        }
    }
}












