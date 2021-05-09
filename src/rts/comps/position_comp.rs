use serde::{Deserialize, Serialize};
use crate::pub_types::PointFloat;
use crate::ecs::ecs_shared::{SerdeObject, Component};

#[derive(Clone, Serialize, Deserialize)]
pub struct PositionComp{
    pub pos: PointFloat,
}

impl SerdeObject for PositionComp{
    fn my_clone(&self) -> Box<dyn SerdeObject> {
        Box::new(self.clone())
    }
    fn my_ser(&self) -> Vec<u8> {
        return bincode::serialize(self).unwrap();
    }
}