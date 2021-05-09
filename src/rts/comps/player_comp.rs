use serde::{Deserialize, Serialize};
use crate::pub_types::PointFloat;
use crate::ecs::ecs_shared::{SerdeObject, Component};
use crate::netcode::InputState;

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerComp {
    pub inputs: InputState,
}
impl Component for PlayerComp{

}

impl SerdeObject for PlayerComp{
    fn my_clone(&self) -> Box<dyn SerdeObject> {
        Box::new(self.clone())
    }
    fn my_ser(&self) -> Vec<u8> {
        return bincode::serialize(self).unwrap();
    }
}