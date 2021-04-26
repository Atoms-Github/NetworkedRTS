use serde::*;
pub type CompositionID = usize;

pub type VerticalStorage<T> = Vec<Vec<T>>;
//pub type TypeSetSerializable = BTreeSet<u64>;
pub type TypeSet = BTreeSet<TypeIdNum>;



use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeSet, BTreeMap};
use std::any::{Any, TypeId};
use anymap::any::CloneAny;
use std::fmt::Debug;
use std::hash::Hash;
use crate::utils::TypeIdNum;
use anymap::AnyMap;
use crate::ecs::{Ecs, Component, System};
use crate::ecs::systems_man::SystemsMan;


struct SlicePointer{
    composition_id: CompositionID,
    index_within_composition: usize
}


// #[derive(Clone, Serialize, Deserialize, Debug)]
#[derive(Serialize, Deserialize)]
pub struct HolyEcs {
    vertical_storages: BTreeMap<TypeIdNum, VerticalStorage<Box<dyn Component>>>
}
impl Clone for HolyEcs{
    fn clone(&self) -> Self {
        let mut cloned : BTreeMap<TypeIdNum, VerticalStorage<Box<dyn Component>>> = BTreeMap::default();
        // Optimum. Optimise. Optimisable.
        for (key, value) in self.vertical_storages.iter(){
            for list in value{
                // breaking.
            }

            //cloned.insert(*key, value.iter().map(|item|{item.iter().map(|item| {item.my_clone()})}).collect());
        }
        Self{
            vertical_storages: cloned
        }
    }
}
impl HolyEcs {
    pub fn new() -> Self{
        return HolyEcs{
            vertical_storages: Default::default()
        };
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







