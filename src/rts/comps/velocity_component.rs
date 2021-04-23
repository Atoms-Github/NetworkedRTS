use crate::ecs::ecs_store::Component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VelocityComponent{

}
#[typetag::serde]
impl Component for VelocityComponent{

}