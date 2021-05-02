use serde::{Deserialize, Serialize};
use crate::pub_types::PointFloat;
use crate::ecs::my_anymap::PlainData;

#[derive(Clone, Serialize, Deserialize)]
pub struct PositionComp{
    pub pos: PointFloat,
}
#[typetag::serde]
impl PlainData for PositionComp{
    fn my_clone(&self) -> Box<dyn PlainData> {
        Box::new(self.clone())
    }
}