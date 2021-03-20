use serde::*;

trait System{
    fn run();
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct EcsMan{

}

impl EcsMan{

}