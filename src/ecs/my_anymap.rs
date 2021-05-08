use std::any::*;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::ecs::{ActiveEcs, Ecs};
use crate::ecs::ecs_shared::{SerdeObject};
use crate::utils::TypeIdNum;
//
//

#[derive(Serialize, Deserialize, Clone)]
pub struct MyAnyMap {
    // data: BTreeMap<TypeIdNum, TOH>,
}
impl MyAnyMap{
    pub fn new() -> Self{
        Self{

        }
    }
}

// impl MyAnyMap {
//     /// Construct a new `AnyMap`.
//     pub fn new() -> Self {
//         Self {
//             data: BTreeMap::new(),
//         }
//     }
// }
//
// impl MyAnyMap {
//     /// Retrieve the value stored in the map for the type `T`, if it exists.
//     // pub fn get<T: 'static>(&self) -> Option<&T> {
//     //     if let Some(any) = self.data.get(&TypeId::of::<T>()) {
//     //         let found : &mut T = any.downcast_mut::<T>().expect("Stored wrong type in typemap somehow.");
//     //         return Some(found);
//     //     }
//     //     return None;
//     // }
//
//     /// Retrieve a mutable reference to the value stored in the map for the type `T`, if it exists.
//     pub fn get_mut<T: SerdeObject + 'static>(&mut self) -> Option<&mut T> {
//         if let Some(any) = self.data.get_mut(&crate::utils::crack_type_id::<T>()) {
//             let found : &mut T = any.data.downcast_mut::<T>().expect("Stored wrong type in typemap somehow.");
//             return Some(found);
//         }
//         return None;
//     }
//
//     /// Set the value contained in the map for the type `T`.
//     /// This will override any previous value stored.
//     pub fn insert<T: SerdeObject + 'static>(&mut self, value: T) {
//         self.data.insert(crate::utils::crack_type_id::<T>(), TOH::new(value));
//     }
//
//     /// Remove the value for the type `T` if it existed.
//     pub fn remove<T: SerdeObject + 'static>(&mut self) {
//         self.data.remove(&crate::utils::crack_type_id::<T>());
//     }
// }