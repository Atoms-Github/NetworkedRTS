use crate::ecs::GlobalEntityID;
use crate::pub_types::PointFloat;

pub enum TargetInstance{
    SINGLE_UNIT(GlobalEntityID),
    FLOOR(PointFloat),
    TWO_POINTS(PointFloat, PointFloat)
}


// TODO: Read notes.