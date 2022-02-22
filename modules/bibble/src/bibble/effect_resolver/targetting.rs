use crate::*;
use game::pub_types::PointFloat;

pub enum TargetInstance{
    SINGLE_UNIT(GlobalEntityID),
    FLOOR(PointFloat),
    TWO_POINTS(PointFloat, PointFloat)
}


// TODO: Read notes.
