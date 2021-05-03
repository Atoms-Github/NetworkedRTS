use crate::ecs::{ActiveEcs, Ecs};
use serde::{Deserialize, Serialize, Serializer};

#[typetag::serde(tag = "type")]
pub trait System : Send{
    fn run(&self, ecs : &mut ActiveEcs);
    fn my_clone(&self) -> Box<dyn System>;
}

pub trait Component : Send{

}


#[derive(Serialize, Deserialize)]
pub struct TOH { // (Trait Object Holder). TODO: Try with just single member. So maybe can do TOH.0.
    #[serde(with = "serde_traitobject")]
    pub data: Box<dyn SerdeObject>,
}
impl TOH {
    pub fn new<T : SerdeObject>(object: T) -> Self{
        Self{
            data: Box::new(object),
        }
    }
}

pub trait SerdeObject: mopa::Any + Send + serde_traitobject::Serialize + serde_traitobject::Deserialize{
    fn my_clone(&self) -> Box<dyn SerdeObject>;
}

mopa::mopafy!(SerdeObject);