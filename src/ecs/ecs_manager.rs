use serde::{Deserialize, Serialize};
use crate::ecs::ecs_store::*;
use std::fmt::Debug;
use std::hash::Hash;
use anymap::any::CloneAny;
use std::any::{TypeId, Any};
use std::collections::HashMap;


pub trait SystemTrait {
    fn gogo(&self) -> i32;
}
pub struct SystemInfo {
    my_system: Box<dyn SystemTrait>
}

struct VelSystem{
}
impl SystemTrait for VelSystem {
    fn gogo(&self) -> i32 {
        return 2;
    }
}
struct AccSystem{
}
impl SystemTrait for AccSystem {
    fn gogo(&self) -> i32 {
        return 2;
    }
}
struct ResourcesHolder {
    items: Vec<SystemInfo>
}

pub fn test_fn(){
    let mut hold = ResourcesHolder {
        items: vec![]
    };

    hold.items.push(SystemInfo{ my_system: Box::new(VelSystem{}) });
    hold.items.push(SystemInfo{ my_system: Box::new(AccSystem{}) });
}
fn testing(res: ResourcesHolder){
    for item in res.items{
        item.my_system.gogo();
    }
}








// #[derive(Clone, Serialize, Deserialize, Hash)]
pub struct EcsManager {
    systems: Vec<Box<dyn System>>,
    root_storage: EcsStore,
}

impl EcsManager {
    pub fn new() -> Self{
        Self{
            systems: vec![],
            root_storage: EcsStore::new()
        }
    }
    // pub fn register_system(&mut self, system: T){
    //     self.systems.push(Box::new(system));
    // }
    pub fn sim(&mut self){
        // TODO3: Can optimise with multithread.
        for system in &mut self.systems{
            system.run(&mut self.root_storage);
        }
    }
}

pub trait System{
    fn run(&mut self, data: &mut EcsStore);
    //fn test() -> i32 {return 2;}
}
// pub trait System : Clone +  Serialize + Deserialize<'static> + Debug + Hash{
//     fn run(&mut self, data: &mut EcsStore);
// }

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct EcsStore{
    // pub vertical_storages: HashMap<TypeId, VerticalStorage<Box<dyn CloneAny>>> // TODO: Test with just a single AnyMap thing. Does it work generics?

}

impl EcsStore{
    pub fn new() -> Self{
        Self{
            // vertical_storages: Default::default()
        }
    }
    pub fn get_component<T>(&self, entity_id: GlobalEntityID){

    }
    pub fn init_component_type<T>(&mut self){
        // self.vertical_storages.insert(T::type_id(), vec![]);
    }
}


