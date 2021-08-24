

use std::any::{TypeId, Any};
use std::collections::HashMap;
use serde::*;

use crate::rts::compsys::*;

use crate::utils::*;
use serde::de::DeserializeOwned;
use crate::ecs::comp_store::*;
use serde::ser::SerializeStruct;
use serde::de::Visitor;
use std::fmt::Write;
use std::fmt;
use super::comp_registration::*;



#[derive(Debug)]
pub struct SuperAny {
    pub boxed: Box<dyn Any + Send>,
}
impl SuperAny {
    pub fn new<T : 'static + Send>(item: T) -> Self{
        let block_box = Self{
            boxed: Box::new(item),
        };
        return block_box;
    }
    pub fn get<T : 'static>(&self) -> &T{
        return (*self.boxed).downcast_ref::<T>().unwrap();
    }
    pub fn get_mut<T : 'static>(&mut self) -> &mut T{
        return (*self.boxed).downcast_mut::<T>().unwrap();
    }
    pub fn get_contained_type(&self) -> TypeIdNum{
        crate::utils::crack_type_id((*self.boxed).type_id())
    }
}
impl Clone for SuperAny {
    fn clone(&self) -> Self {
        let functions = FUNCTION_MAP.get_from_type_id((*self.boxed).type_id());
        return SuperAny{
            boxed: (functions.do_clone)(&self.boxed)
        };
    }
}


#[derive(Serialize, Clone, Deserialize)]
struct SuperAnyPortable{
    type_id: TypeIdNum,
    bytes: Vec<u8>,
}
impl Serialize for SuperAny{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let functions = FUNCTION_MAP.get_from_type_id((*self.boxed).type_id());
        let portable = SuperAnyPortable{
            bytes: (functions.ser)(&self.boxed),
            type_id: crate::utils::crack_type_id((*self.boxed).type_id()),
        };
        portable.serialize(serializer)
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

        let functions = FUNCTION_MAP.get(portable.type_id);

        let item = (functions.deser)(&portable.bytes);

        return Ok(Self{
            boxed: item
        });
    }
}