use crate::ecs::{Ecs, ActiveEcs};

#[typetag::serde(tag = "type")]
pub trait System : Send{
    fn run(&self, ecs : &mut ActiveEcs);
    fn my_clone(&self) -> Box<dyn System>;
}