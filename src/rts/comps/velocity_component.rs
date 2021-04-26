use serde::{Deserialize, Serialize};
use crate::ecs::ecs_shared::Component;
use crate::pub_types::PointFloat;

#[derive(Clone, Serialize, Deserialize)]
pub struct VelocityComp {
    pub vel: PointFloat,
}
#[typetag::serde]
impl Component for VelocityComp {
    fn my_clone(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}