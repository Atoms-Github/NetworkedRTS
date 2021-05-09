use serde::{Deserialize, Serialize};
use crate::pub_types::{PointFloat, PlayerID};
use crate::ecs::ecs_shared::{SerdeObject, Component};
use crate::ecs::GlobalEntityID;

#[derive(Clone, Serialize, Deserialize)]
pub struct OwnedComp {
    pub owner: GlobalEntityID,
}
impl Component for OwnedComp {

}

impl SerdeObject for OwnedComp {
    fn my_clone(&self) -> Box<dyn SerdeObject> {
        Box::new(self.clone())
    }
    fn my_ser(&self) -> Vec<u8> {
        return bincode::serialize(self).unwrap();
    }
}