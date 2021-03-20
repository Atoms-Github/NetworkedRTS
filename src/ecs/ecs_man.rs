use serde::*;

pub trait System{
    type SystemData: DynamicSystemData;

    fn run(data: Self::SystemData);
}
trait DynamicSystemData{
    fn get_me(manager: &EcsMan) -> Self;
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct EcsMan{

}

impl EcsMan{
    pub fn new() -> Self{
        Self{

        }
    }
}