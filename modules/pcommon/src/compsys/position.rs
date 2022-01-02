use game::pub_types::PointFloat;
use bibble::::*;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PositionComp{
    pub pos: PointFloat,
}