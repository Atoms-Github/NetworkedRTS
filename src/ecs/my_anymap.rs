use std::any::*;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::BTreeMap;
use crate::utils::TypeIdNum;
use crate::ecs::{Ecs, ActiveEcs};


#[typetag::serde(tag = "type")]
pub trait PlainData : mopa::Any + Send{
    fn my_clone(&self) -> Box<dyn PlainData>;
}
mopa::mopafy!(PlainData);

#[derive(Serialize, Deserialize)]
pub struct MyAnyMap {
    data: BTreeMap<TypeIdNum, Box<dyn PlainData>>,
}

impl MyAnyMap {
    /// Construct a new `AnyMap`.
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }
}

impl MyAnyMap {
    /// Retrieve the value stored in the map for the type `T`, if it exists.
    // pub fn get<T: 'static>(&self) -> Option<&T> {
    //     if let Some(any) = self.data.get(&TypeId::of::<T>()) {
    //         let found : &mut T = any.downcast_mut::<T>().expect("Stored wrong type in typemap somehow.");
    //         return Some(found);
    //     }
    //     return None;
    // }

    /// Retrieve a mutable reference to the value stored in the map for the type `T`, if it exists.
    pub fn get_mut<T: PlainData + 'static>(&mut self) -> Option<&mut T> {
        if let Some(any) = self.data.get_mut(&crate::utils::get_type_id::<T>()) {
            let found : &mut T = any.downcast_mut::<T>().expect("Stored wrong type in typemap somehow.");
            return Some(found);
        }
        return None;
    }

    /// Set the value contained in the map for the type `T`.
    /// This will override any previous value stored.
    pub fn insert<T: PlainData + 'static>(&mut self, value: T) {
        self.data.insert(crate::utils::get_type_id::<T>(), Box::new(value));
    }

    /// Remove the value for the type `T` if it existed.
    pub fn remove<T: PlainData + 'static>(&mut self) {
        self.data.remove(&crate::utils::get_type_id::<T>());
    }
}