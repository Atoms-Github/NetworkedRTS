use crate::pub_types::PointFloat;
use crate::rts::compsys::*;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PositionComp{
    pub pos: PointFloat,
}