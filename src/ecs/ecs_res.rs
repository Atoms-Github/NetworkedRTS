use std::collections::HashMap;
use crate::ecs::ecs_manager::System;
use crate::rts::systems::velocity_system::VelSystem;

pub type SystemId = usize;

pub struct EcsRes{
    pub systems_lookup: HashMap<SystemId, Box<dyn System>>,
    next_system_id : usize
}

impl EcsRes{
    // pub fn get_system(&self, system_id: SystemId) -> &dyn System{
    //     return &self.systems_lookup.get(&system_id).unwrap();
    // }
    fn add_sys<T: 'static + System>(&mut self, system: T){
        self.systems_lookup.insert(self.next_system_id, Box::new(system));
        self.next_system_id += 1;
    }
    pub fn new() -> Self{
        let mut me = Self{
            systems_lookup: Default::default(),
            next_system_id: 0
        };
        me.add_sys(VelSystem{});

        return me;
    }
}