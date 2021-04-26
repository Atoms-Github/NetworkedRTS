use crate::ecs::{Ecs, ActiveEcs};

#[typetag::serde(tag = "type")]
pub trait System : Send{
    fn run(&self, ecs : &ActiveEcs);
    fn my_clone(&self) -> Box<dyn System>;
}


#[typetag::serde(tag = "type")]
pub trait Component : mopa::Any + Send{
    fn my_clone(&self) -> Box<dyn Component>;
}

mopa::mopafy!(Component);


// TODO:  System and component macros.