use serde::{Deserialize, Serialize};
use crate::pub_types::PointFloat;
use crate::ecs::my_anymap::PlainData;

#[derive(Clone, Serialize, Deserialize)]
pub struct VelocityComp {
    pub vel: PointFloat,
}
#[typetag::serde]
impl PlainData for VelocityComp {
    fn my_clone(&self) -> Box<dyn PlainData> {
        Box::new(self.clone())
    }
}