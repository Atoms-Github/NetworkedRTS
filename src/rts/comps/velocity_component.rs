use serde::{Deserialize, Serialize};
use crate::ecs::ecs_shared::Component;

#[derive(Clone, Serialize, Deserialize)]
pub struct VelocityComponent{

}
#[typetag::serde]
impl Component for VelocityComponent{
    fn my_clone(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}