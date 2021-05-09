use crate::ecs::{ActiveEcs, Ecs};
use serde::{Deserialize, Serialize, Serializer};
use crate::utils::TypeIdNum;
use serde::de::DeserializeOwned;

#[typetag::serde(tag = "type")]
pub trait System : Send{
    fn run(&self, ecs : &mut ActiveEcs);
    fn my_clone(&self) -> Box<dyn System>;
}

pub trait Component : Send + Serialize + Clone{

}



pub trait SerdeObject: mopa::Any + Send{
    fn my_clone(&self) -> Box<dyn SerdeObject>;
    fn my_ser(&self) -> Vec<u8>;
}

mopa::mopafy!(SerdeObject);



// #[derive(Serialize)]
// pub struct World{
//     pub positions: Vec<PositionComponent>,
//     pub velocities: Vec<VelocityComponent>,
//     pub accelerations: Vec<AccelerationComponent>,
//
// }


