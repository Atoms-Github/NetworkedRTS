use serde::{Deserialize, Serialize};
use crate::pub_types::PointFloat;
use crate::ecs::ecs_shared::{SerdeObject, Component};

#[derive(Clone, Serialize, Deserialize)]
pub struct PositionComp{
    pub pos: PointFloat,
}

impl Component for PositionComp{

}
