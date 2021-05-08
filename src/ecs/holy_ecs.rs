use std::any::{Any, TypeId};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

use anymap::any::CloneAny;
use anymap::AnyMap;
use serde::{Deserialize, Serialize, Serializer};

use crate::ecs::{Ecs, System};
use crate::ecs::ecs_shared::{SerdeObject, Component};
use crate::ecs::my_anymap::MyAnyMap;
use crate::ecs::systems_man::SystemsMan;
use crate::utils::TypeIdNum;

pub type CompositionID = usize;

pub type VerticalStorage<T> = Vec<Vec<T>>;
//pub type TypeSetSerializable = BTreeSet<u64>;
pub type TypeSet = BTreeSet<TypeIdNum>;


struct SlicePointer{
    composition_id: CompositionID,
    index_within_composition: usize
}

#[derive(Serialize, Deserialize, Clone)]
struct Column{
    data: Vec<i32>
}

// impl SerdeObject for Column{
//     fn my_clone(&self) -> Box<dyn SerdeObject> {
//         Box::new(self.clone())
//     }
// }

#[derive(Serialize, Deserialize, Clone)]
pub struct HolyEcs {
    storages: MyAnyMap,
}
impl HolyEcs {
    pub fn new() -> Self{
        HolyEcs{
            storages: MyAnyMap::new(),
        }
    }
}
impl Ecs for HolyEcs {


    fn add_entity(&mut self, new_components: AnyMap) -> usize {
        unimplemented!()
    }

    fn query(&self, types: Vec<TypeId>) -> Vec<usize> {
        unimplemented!();
    }

    fn get<T: Component>(&self, entity_id: usize) -> &T {
        unimplemented!()
    }
    fn get_mut<T: Component>(&mut self, entity_id: usize) -> &mut T {
        unimplemented!()
    }

    fn run_systems(&mut self, systems: &SystemsMan) {
        for system in &systems.systems{
            system.run(self);
        }
    }
}







