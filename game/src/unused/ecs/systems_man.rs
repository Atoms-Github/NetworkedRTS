
use serde::*;
use crate::ecs::System;

#[derive(Serialize, Deserialize)]
pub struct SystemsMan{
    pub systems: Vec<Box<dyn System>>,
}

impl Clone for SystemsMan{
    fn clone(&self) -> Self {
        Self{
            systems: self.systems.iter().map(|item|{item.my_clone()}).collect()
        }
    }
}

impl SystemsMan{
    pub fn new() -> Self{
        Self{
            systems: vec![]
        }
    }
    pub fn add_system<T : System + 'static>(&mut self, system: T){
        self.systems.push(Box::new(system));
    }
}