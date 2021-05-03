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

// #[derive(Serialize, Deserialize, Clone)]
// struct Column<T>{
//
// }
// #[typetag::serde]
// impl<T> PlainData for Column<T>{
//     fn my_clone(&self) -> Box<dyn PlainData> {
//         Box::new(self.clone())
//     }
// }

// #[derive(Clone, Serialize, Deserialize, Debug)]
#[derive(Serialize, Deserialize)]
pub struct HolyEcs {
    // vertical_storages: BTreeMap<TypeIdNum, VerticalStorage<Box<dyn SerdeObject>>>,
    test: MyAnyMap,
    // test: AnyMap,
}
impl Clone for HolyEcs{
    fn clone(&self) -> Self {
        let mut cloned : BTreeMap<TypeIdNum, VerticalStorage<Box<dyn SerdeObject>>> = BTreeMap::default();
        // Optimisable.
        // for (key, value) in self.vertical_storages.iter(){
        //     for list in value{
        //         // breaking.
        //     }
        //
        //     //cloned.insert(*key, value.iter().map(|item|{item.iter().map(|item| {item.my_clone()})}).collect());
        // }
        unimplemented!();
        // Self{
        //     vertical_storages: cloned,
        //     test: self.test.clone(),
        // }
    }
}
impl HolyEcs {
    pub fn new() -> Self{
        unimplemented!();
        // return HolyEcs{
        //     vertical_storages: Default::default(),
        //     // test: AnyMap::new(),
        // };
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







