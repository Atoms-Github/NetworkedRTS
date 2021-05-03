use serde::{Deserialize, Serialize};
use crate::pub_types::PointFloat;
use crate::ecs::ecs_shared::{SerdeObject, Component};

#[derive(Clone, Serialize, Deserialize)]
pub struct VelocityComp {
    pub vel: PointFloat,
}

impl Component for VelocityComp{

}