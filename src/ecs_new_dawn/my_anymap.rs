use std::any::*;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::ecs::{ActiveEcs};
use crate::ecs::ecs_shared::{SerdeObject};
use crate::utils::TypeIdNum;
use crate::ecs::liquid_garbage::TOH;
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Clone)]
pub struct SerdeAnyMap {
    pub data: BTreeMap<TypeIdNum, TOH>,
}
impl SerdeAnyMap {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }
}
impl SerdeAnyMap {
    // pub fn get<T: 'static>(&self) -> Option<&T> {
    //     if let Some(any) = self.data.get(&TypeId::of::<T>()) {
    //         let found : &mut T = any.downcast_mut::<T>().expect("Stored wrong type in typemap somehow.");
    //         return Some(found);
    //     }
    //     return None;
    // }

    pub fn contains_key<T: DeserializeOwned + SerdeObject + 'static>(&self) -> bool{
        return self.data.contains_key(&crate::utils::get_type_id::<T>());
    }

    pub fn get_mut<T: DeserializeOwned + SerdeObject + 'static>(&mut self) -> Option<&mut T> {
        if let Some(toh) = self.data.get_mut(&crate::utils::get_type_id::<T>()) {
            let found : &mut T = toh.get::<T>();
            return Some(found);
        }
        return None;
    }
    pub fn insert<T: DeserializeOwned + SerdeObject + 'static>(&mut self, value: T) {
        self.data.insert(crate::utils::get_type_id::<T>(), TOH::new(value));
    }
    pub fn remove<T: DeserializeOwned + SerdeObject + 'static>(&mut self) {
        self.data.remove(&crate::utils::get_type_id::<T>());
    }
}