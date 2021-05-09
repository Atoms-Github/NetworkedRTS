use crate::ecs::{ActiveEcs, Ecs};
use serde::{Deserialize, Serialize, Serializer, Deserializer, de};
use crate::utils::TypeIdNum;
use crate::ecs::ecs_shared::SerdeObject;
use std::any::TypeId;
use mopa::Any;
use anymap::any::CloneAny;
use std::borrow::Borrow;
use serde::de::{DeserializeOwned, Visitor, MapAccess};
use serde::ser::{SerializeTuple, Error};
use std::marker::PhantomData;
use std::fmt::Write;
use std::fmt;

struct TOHVisitor {
}

impl TOHVisitor {
    fn new() -> Self {
        TOHVisitor {}
    }
}
impl<'de> Visitor<'de> for TOHVisitor
{
    type Value = TOH;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A TOH")
    }

    fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
    {
        let type_id = bincode::deserialize::<TypeIdNum>(&bytes[0..8]).unwrap();
        let data = bytes[8..].to_vec(); // No vec wrapper on this - its RAW.

        return Ok(TOH{
            my_type: type_id,
            serable: None,
            blobbity: data
        });
    }
}

impl<'de> Deserialize<'de> for TOH
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(TOHVisitor::new())
    }
}
pub struct TOH { // (Trait Object Holder).
    my_type: TypeIdNum,
    serable: Option<Box<dyn SerdeObject>>,
    blobbity: Vec<u8>,
}
impl Clone for TOH{
    fn clone(&self) -> Self {
        let mut my_option = None;

        if let Some(existing) = self.serable{
            my_option = Some(existing.my_clone());
        }
        return Self{
            my_type: self.my_type,
            serable: my_option,
            blobbity: self.blobbity.clone()
        }
    }
}
impl Serialize for TOH{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut bytes = vec![];
        bytes.extend(bincode::serialize(&self.my_type).unwrap());
        bytes.extend(self.serable.as_ref().unwrap().my_ser());
        serializer.serialize_bytes(&bytes[..])
    }
}

impl TOH {
    pub fn new<T : SerdeObject>(object: T) -> Self{
        Self{
            serable: Some(Box::new(object)),
            blobbity: vec![],
            my_type: crate::utils::crack_type_id::<T>(),
        }
    }
    pub fn get<'a, T : DeserializeOwned + SerdeObject + 'static>(&mut self) -> &mut T{
        assert_eq!(crate::utils::crack_type_id::<T>(), self.my_type);
        if self.serable.is_none(){
            assert!(self.blobbity.len() > 0, "Object had neither serialized version, nor deserialzed version!");
            let result = bincode::deserialize::<T>(&self.blobbity[..]).unwrap().my_clone();
            self.serable = Some(result);
        }
        let value = self.serable.as_mut().unwrap();
        let casted = value.downcast_mut::<T>();
        return casted.unwrap();
    }
}

#[cfg(test)]
mod toh_tests {
    use super::*;

    #[derive(Clone, Serialize, Deserialize)]
    struct TestStruct{
        value_one: i32,
        list: Vec<u8>,
        value_two: u8,
    }

    #[derive(Clone, Serialize, Deserialize)]
    struct TestStructGeneric<T : Clone>{
        value_one: i32,
        value_two: T,
    }
    impl<T: 'static + Send + Clone + Serialize> SerdeObject for TestStructGeneric<T>{
        fn my_clone(&self) -> Box<dyn SerdeObject> {
            return Box::new(self.clone());
        }

        fn my_ser(&self) -> Vec<u8> {
            return bincode::serialize(self).unwrap();
        }
    }
    impl SerdeObject for TestStruct{
        fn my_clone(&self) -> Box<dyn SerdeObject> {
            return Box::new(self.clone());
        }

        fn my_ser(&self) -> Vec<u8> {
            return bincode::serialize(self).unwrap();
        }
    }

    #[test]
    fn basic_serde_test() {
        let original_source = TestStruct{
            value_one: 6,
            list: vec![6,1,2,1],
            value_two: 2,
        };
        let original_toh = TOH::new(original_source);

        let serialized = bincode::serialize(&original_toh).unwrap();

        println!("TOH TypeID: {}, deser {:?}", crate::utils::crack_type_id::<TestStruct>(), serialized);

        let mut deserialized = bincode::deserialize::<TOH>(&serialized).unwrap();
        let casted = deserialized.get::<TestStruct>();

        assert_eq!(casted.list, vec![6,1,2,1]);
        assert_eq!(casted.value_one, 6);
        assert_eq!(casted.value_two, 2);
    }
    #[test]
    fn generic_serde_test() {
        let float : f32 = 10.0;
        let original_source = TestStructGeneric{
            value_one: 6,
            value_two: float,
        };
        let original_toh = TOH::new(original_source);

        let serialized = bincode::serialize(&original_toh).unwrap();


        let mut deserialized = bincode::deserialize::<TOH>(&serialized).unwrap();
        let casted = deserialized.get::<TestStructGeneric<f32>>();

        assert_eq!(casted.value_one, 6);
        assert_eq!(casted.value_two, 10.0);
    }
    // #[test]
    // fn test_serde_vec() {
    //     let original_toh = vec![2];
    //
    //     let serialized = bincode::serialize(&original_toh).unwrap();
    //
    //     println!("TOH TypeID:deser {:?}", serialized);
    //     assert_eq!(casted.value_two, 2);
    // }
}