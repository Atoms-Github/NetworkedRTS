use std::any::{TypeId, Any};
use std::collections::HashMap;
use serde::*;

use crate::rts::compsys::*;

use crate::utils::*;
use serde::de::DeserializeOwned;
use crate::ecs::comp_store::*;
use serde::ser::SerializeStruct;
use serde::de::Visitor;
use std::fmt::{Write, Debug};
use std::fmt;
use super::comp_registration::*;
use crate::ecs::bblocky::super_any::SuperAny;
use std::clone::Clone;
use crate::unsafe_utils::very_bad_function;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::bibble::data::data_types::__private::Formatter;
use crate::ecs::superb_ecs::EcsConfig;
use std::marker::PhantomData;


#[derive(PartialEq)]
pub struct SuperVec<C> {
    phantom: PhantomData<C>,
    item_size: usize,
    data: Vec<u8>,
    item_type: TypeIdNum,
    debug_name: String,
}
impl<C> Debug for SuperVec<C>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let functions = C::get_functions().get(self.item_type);
        let mut items = vec![];
        for i in 0..self.len(){
            let bytes = self.get_as_bytes(i);
            let mut debug_string = (functions.debug_fmt)(bytes);
            items.push(debug_string);
        }
        f.debug_struct(format!("SuperVec of {}", &self.debug_name).as_str())
            .field("item_size", &self.item_size)
            .field("item_type", &self.item_type)
            .field("data", &items.join(", "))
            .finish()
    }
}


impl<C> SuperVec<C> {
    pub fn new(item_type: TypeIdNum) -> Self{
        let functions = C::get_functions().get(item_type);
        Self{
            item_size: functions.item_size,
            data: vec![],
            item_type,
            debug_name: functions.debug_name.clone()
        }
    }
    pub fn new_and_push<T : 'static + Send>(item: T) -> Self{
        let mut vec = Self::new(gett::<T>());
        vec.push(item);
        return vec;
    }
    pub fn len(&self) -> usize{
        assert_eq!(self.data.len() % self.item_size, 0);
        return self.data.len() / self.item_size;
    }
    pub fn push_super_any(&mut self, mut item: SuperAny<C>){
        assert_eq!(self.item_size, item.list.item_size);
        assert_eq!(self.item_type, item.list.item_type);
        let mut bytes = item.list.move_as_bytes(0);
        self.data.append(&mut bytes);
    }
    pub fn push<T : 'static>(&mut self, item: T){ // Just push absolutely anything you want.
        assert_eq!(gett::<T>(), self.item_type);
        let as_slice = unsafe{crate::unsafe_utils::struct_as_u8_slice(&item)};
        let mut as_bytes = as_slice.to_vec();
        assert_eq!(as_bytes.len(), self.item_size);
        self.data.append(&mut as_bytes);
        std::mem::forget(item);
    }
    pub fn get_as_bytes(&self, index: usize) -> &[u8]{
        return &self.data[index * self.item_size..(index + 1) * self.item_size];
    }
    pub fn move_as_bytes(&mut self, index: usize) -> Vec<u8>{
        assert!(index < self.len());
        return self.data.drain(index * self.item_size..(index + 1) * self.item_size).collect();
    }
    pub fn swap_remove(&mut self, index: usize){
        // If we can simply just normal remove off the end.
        if index == self.len() - 1{
            self.drop_items_refs(index);
            self.data.drain((self.data.len() - self.item_size)..);
        }else{
            self.drop_items_refs(index);
            let swap_source_index = self.data.len() - self.item_size;
            let swap_target_index = index * self.item_size;
            let my_data : Vec<u8> = self.data.drain(swap_source_index..swap_source_index + self.item_size).collect();
            // Splice if in middle, otherwise just remove from end.
            self.data.splice(swap_target_index..swap_target_index + self.item_size, my_data);
        }
    }
    /// Properly deallocates all data referenced to by the item in position INDEX.
    pub fn drop_items_refs(&self, index: usize){
        let functions = FUNCTION_MAP.get(self.item_type);
        (functions.deallocate_refed_mem)(self.get_as_bytes(index));
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
    pub fn get_mut<T : 'static>(&mut self, index: usize) -> Option<&mut T>{
        return self.get(index).map(|item| unsafe {very_bad_function(item)});
    }
}
impl<C : EcsConfig> Clone for SuperVec<C> {
    fn clone(&self) -> Self {
        let functions = C::get_functions().get(self.item_type);
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
            debug_name: self.debug_name.clone(),
        }
    }
}
impl<C : EcsConfig> Hash for SuperVec<C>{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let functions = C::get_functions().get(self.item_type);
        for i in 0..self.len(){
            let bytes = self.get_as_bytes(i);
            let serialized_bytes = (functions.meme_ser)(bytes);
            serialized_bytes.hash(state);
        }
    }
}
impl<C> Drop for SuperVec<C>{
    fn drop(&mut self) {
        // Dealocate all memory referenced.
        for i in 0..self.len(){
            self.drop_items_refs(i);
        }
    }
}
#[derive(Serialize, Clone, Deserialize)]
struct SuperVecPortable{
    item_size_when_deser: usize,
    data: Vec<Vec<u8>>,
    item_type_when_deser: TypeIdNum,
}
impl<C : EcsConfig> Serialize for SuperVec<C>{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let functions = C::get_functions().get(self.item_type);

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
impl<'de, C : EcsConfig> Deserialize<'de> for SuperVec<C>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let portable = SuperVecPortable::deserialize(deserializer).unwrap();

        let functions = C::get_functions().get(portable.item_type_when_deser);
        let mut data = vec![];
        for serialized in portable.data{
            let mut forgotten_item = (functions.meme_deser_and_forget)(&serialized);
            data.append(&mut forgotten_item);
        }
        return Ok(Self{
            item_size: portable.item_size_when_deser,
            data,
            item_type: portable.item_type_when_deser,
            debug_name: functions.debug_name.clone(),
        });
    }
}
