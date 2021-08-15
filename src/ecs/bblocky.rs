
use std::any::{TypeId, Any};
use std::collections::HashMap;
use serde::*;

use lazy_static::lazy_static;
use crate::utils::*;
use serde::de::DeserializeOwned;
use crate::ecs::comp_store::*;
use serde::ser::SerializeStruct;
use serde::de::Visitor;
use std::fmt::Write;
use std::fmt;

lazy_static! {
    static ref FUNCTION_MAP: HashMap<TypeIdNum, SuperbFunctions> = {
        let mut map = HashMap::new();
        // register_type::<ATestStruct>(&mut map);
        map
    };
}

fn register_type<T : 'static + Serialize + Clone + DeserializeOwned>(map: &mut HashMap<TypeIdNum, SuperbFunctions>){
    map.insert(gett::<T>(), SuperbFunctions {
        do_clone: |item| {
            let casted = (*item).downcast_ref::<T>().unwrap();
            Box::new(casted.clone())
        },
        ser: |item| {
            let casted = (*item).downcast_ref::<T>().unwrap();
            return bincode::serialize(casted).unwrap();
        },
        deser: |bytes| {
            let item = bincode::deserialize::<T>(bytes).unwrap();
            return Box::new(item);
        },
    });
}
struct SuperbFunctions {
    do_clone: fn(&Box<dyn Any>) -> Box<dyn Any>,
    ser: fn(&Box<dyn Any>) -> Vec<u8>,
    deser: fn(&Vec<u8>) -> Box<dyn Any>,
}
impl SuperbFunctions{
    pub fn get_from_type_id(type_id: TypeId) -> &'static Self{
        return Self::get(crack_type_id(type_id));
    }
    pub fn get(type_id_num: TypeIdNum) -> &'static Self{
        return FUNCTION_MAP.get(&type_id_num).expect("Type wasn't registered!");
    }
}
struct SuperAny {
    item: Box<dyn Any>,
}
impl SuperAny {
    pub fn new<T : 'static>(item: T) -> Self{
        let block_box = Self{
            item: Box::new(item),
        };
        return block_box;
    }
}
impl Clone for SuperAny {
    fn clone(&self) -> Self {
        let functions = SuperbFunctions::get_from_type_id(self.item.type_id());
        return SuperAny{
            item: Box::new((functions.do_clone)(&self.item))
        };
    }
}

#[cfg(test)]
mod ecs_tests {
    use super::*;
    #[test]
    fn testing() {
        let strb = TestStructB{
            integer: 0,
            vec: vec![vec![], vec![TestStructA{
                integer: 0,
                float: 0.0,
                vec: vec![8,5]
            }]],
            float: 0.0
        };
        let e = 3;

    }
    #[derive(Serialize, Deserialize, Clone)]
    struct TestStructA{
        integer: u32,
        float: f32,
        vec: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Clone)]
    struct TestStructB{
        integer: u32,
        vec: Vec<Vec<TestStructA>>,
        float: f32,
    }
}

#[derive(Serialize, Clone, Deserialize)]
struct SuperAnyPortable{
    bytes: Vec<u8>,
    type_id: TypeIdNum,
}
impl Serialize for SuperAny{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let functions = SuperbFunctions::get_from_type_id(self.item.type_id());
        let bytes = (functions.ser)(&self.item);
        let portable = SuperAnyPortable{
            bytes,
            type_id: crate::utils::crack_type_id(self.item.type_id()),
        };
        serializer.serialize_bytes(bincode::serialize(&portable).unwrap().as_slice())
    }
}
struct SuperAnyVisitor {}
impl<'de> Deserialize<'de> for SuperAny
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let portable = SuperAnyPortable::deserialize(deserializer).unwrap();

        let functions = SuperbFunctions::get(portable.type_id);

        let item = (functions.deser)(&portable.bytes);

        return Ok(Self{
            item: Box::new(item)
        });
    }
}



//
// ///
// /// What we need to do with this:
// /// A. get() it out of pending entity. (Tick)
// /// B. Get its type ID. (Tick)
// /// C. Get it as bytes.
// ///
// struct BlockBox{ // Struct for transport. (E.g. in pending entity).
//     item: Box<dyn Any>,
//     size: usize,
// }
// impl BlockBox{
//     pub fn new<T : 'static>(item: T) -> Self{
//         let block_box = Self{
//             item: Box::new(item),
//             size: std::mem::size_of::<T>(),
//         };
//         return block_box;
//     }
// }


// struct Block{
//     item_size: usize,
//     data: Vec<u8>,
//     item_type: TypeIdNum,
// }
// struct SerBlock{
//     item_size: usize,
//     data: Vec<u8>,
//     item_type: TypeIdNum,
// }
// impl Clone for Block{
//     fn clone(&self) -> Self {
//         todo!()
//     }
// }
// impl Block{
//     pub fn new() -> Self{
//         Self{
//             item_size: 0,
//             data: vec![],
//             item_type: 0
//         }
//     }
//     pub fn len(&self) -> usize{
//         assert!(self.data.len() % self.item_size == 0);
//         return self.data.len() / self.item_size;
//     }
//     pub fn push(&mut self, item: BlockBox){
//         assert_eq!(crate::utils::break_open_type_id(item.type_id()), self.item_type)
//
//     }
// }