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
use crate::ecs::bblocky::super_any::SuperAny;


pub struct SuperVec {
    item_size: usize,
    data: Vec<u8>,
    item_type: TypeIdNum,
}


impl SuperVec {
    pub fn new_from_any(example_item: &SuperAny) -> Self{
        Self::new(example_item.get_contained_type())
    }
    pub fn new(item_type: TypeIdNum) -> Self{
        let functions = FUNCTION_MAP.get(item_type);
        Self{
            item_size: functions.item_size,
            data: vec![],
            item_type
        }
    }
    pub fn len(&self) -> usize{
        assert_eq!(self.data.len() % self.item_size, 0);
        return self.data.len() / self.item_size;
    }
    pub fn push(&mut self, item: SuperAny){
        assert_eq!(item.get_contained_type(), self.item_type);
        let reff = &*item.boxed;
        let as_slice = unsafe{crate::unsafe_utils::struct_as_u8_slice(reff)};
        let mut as_bytes = as_slice.to_vec();
        assert_eq!(as_bytes.len(), self.item_size);
        self.data.append(&mut as_bytes);
        std::mem::forget(item);
    }
    pub fn get_as_bytes(&self, index: usize) -> &[u8]{
        return &self.data[index * self.item_size..(index + 1) * self.item_size];
    }
    pub fn get<T : 'static>(&self, index: usize) -> Option<&T>{
        assert_eq!(gett::<T>(), self.item_type);
        if self.len() <= index{
            return None;
        }else{
            let value = unsafe{crate::unsafe_utils::u8_slice_to_ref(self.get_as_bytes(index))};
            return Some(value);
        }
    }
}
impl Clone for SuperVec {
    fn clone(&self) -> Self {
        let functions = FUNCTION_MAP.get(self.item_type);
        let mut data = vec![];
        for i in 0..self.len(){
            let bytes = self.get_as_bytes(i);
            let mut cloned_forgotten_bytes = (functions.meme_clone_and_forget)(bytes);
            data.append(&mut cloned_forgotten_bytes);
        }
        return Self{
            item_size: self.item_size,
            data,
            item_type: self.item_type,
        }
    }
}
impl Drop for SuperVec{
    fn drop(&mut self) {
        todo!()
    }
}
#[derive(Serialize, Clone, Deserialize)]
struct SuperVecPortable{
    item_size_when_deser: usize,
    data: Vec<Vec<u8>>,
    item_type_when_deser: TypeIdNum,
}
impl Serialize for SuperVec{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let functions = FUNCTION_MAP.get(self.item_type);
        let mut items = vec![];
        for i in 0..self.len(){
            let bytes = self.get_as_bytes(i);
            let serialized_bytes = (functions.meme_ser)(bytes);
            items.push(serialized_bytes);
        }
        let portable = SuperVecPortable{
            item_size_when_deser: self.item_size,
            data: items,
            item_type_when_deser: self.item_type
        };
        portable.serialize(serializer)
    }
}

struct SuperVecVisitor {}
impl<'de> Deserialize<'de> for SuperVec
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let portable = SuperVecPortable::deserialize(deserializer).unwrap();

        let functions = FUNCTION_MAP.get(portable.item_type_when_deser);
        let mut data = vec![];
        for serialized in portable.data{
            let mut forgotten_item = (functions.meme_deser_and_forget)(&data);
            data.append(&mut forgotten_item);
        }
        return Ok(Self{
            item_size: portable.item_size_when_deser,
            data,
            item_type: portable.item_type_when_deser,
        });
    }
}
