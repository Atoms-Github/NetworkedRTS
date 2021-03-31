use crate::ecs::ecs_manager::System;

pub struct SystemInfo {
    pub my_system: Box<dyn System>,
    pub name: String,
}
pub struct SystemsLookup {
    pub items: Vec<SystemInfo>
}
impl SystemsLookup{
    pub fn new() -> Self{
        SystemsLookup{
            items: vec![]
        }
    }
    pub fn add_sys<T : 'static + System>(&mut self, system: T){
        self.items.push(SystemInfo{
            my_system: Box::new(system),
            name: "".to_string()
        });
    }
}

