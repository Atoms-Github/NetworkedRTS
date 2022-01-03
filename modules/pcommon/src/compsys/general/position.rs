use crate::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PositionComp{
    pub pos: PointFloat,
}