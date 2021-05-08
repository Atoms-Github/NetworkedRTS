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
// This is the trait that Deserializers are going to be driving. There
// is one method for each type of data that our type knows how to
// deserialize from. There are many other methods that are not
// implemented here, for example deserializing from integers or strings.
// By default those methods will return an error, which makes sense
// because we cannot deserialize a MyMap from an integer or string.
impl<'de> Visitor<'de> for TOHVisitor
{
    // The type that our Visitor is going to produce.
    type Value = TOH;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        println!("Expecting!");
        formatter.write_str("A TOH")
    }

    fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
    {
        println!("VisitingBytes!");
        let type_id = bincode::deserialize::<TypeIdNum>(&bytes[0..4]).unwrap();
        println!("InVisitBytes deser typeId: {}", type_id);
        let data = bincode::deserialize::<Vec<u8>>(&bytes[4..]).unwrap();

        return Ok(TOH{
            my_type: type_id,
            serable: None,
            blobbity: data
        });
    }
}

// This is the trait that informs Serde how to deserialize MyMap.
impl<'de> Deserialize<'de> for TOH
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of MyMap.
        deserializer.deserialize_bytes(TOHVisitor::new())
    }
}
pub struct TOH { // (Trait Object Holder). TODO: Add type check.
    my_type: TypeIdNum,
    serable: Option<Box<dyn SerdeObject>>,
    blobbity: Vec<u8>,
}
impl Serialize for TOH{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut bytes = vec![];
        bytes.extend(bincode::serialize(&self.my_type).unwrap());
        bytes.extend(self.serable.as_ref().unwrap().my_ser());
        serializer.serialize_bytes(&bytes[..])
        // let mut tup = serializer.serialize_tuple(2)?;
        // tup.serialize_element(&self.my_type)?;
        // tup.serialize_element(&self.serable.as_ref().unwrap().my_ser())?;
        // serializer.serialize_bytes()
        // return tup.end();
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
    pub fn get<'a, T : DeserializeOwned + SerdeObject + 'static>(&mut self) -> &T{
        assert!(crate::utils::crack_type_id::<T>() == self.my_type);
        if self.serable.is_none(){
            assert!(self.blobbity.len() > 0, "Object had neither serialized version, nor deserialzed version!");
            let result = bincode::deserialize::<T>(&self.blobbity[..]).unwrap().my_clone();
            self.serable = Some(result);
        }

        return &self.serable.as_ref().unwrap().downcast_ref::<T>().unwrap();
    }
}

#[cfg(test)]
mod toh_tests {
    use super::*;

    #[derive(Clone, Serialize, Deserialize)]
    struct TestStruct{
        // value_one: i32,
        // list: Vec<u8>,
        value_two: u8,
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
            // value_one: 6,
            // list: vec![6,1,2,1],
            value_two: 2,
        };
        let original_toh = TOH::new(original_source);

        let serialized = bincode::serialize(&original_toh).unwrap();

        println!("TOH TypeID: {}, deser {:?}", crate::utils::crack_type_id::<TestStruct>(), serialized);

        let mut deserialized = bincode::deserialize::<TOH>(&serialized).unwrap();
        let casted = deserialized.get::<TestStruct>();

        // assert_eq!(casted.list, vec![6,1,2,1]);
        // assert_eq!(casted.value_one, 6);
        assert_eq!(casted.value_two, 2);
    }
}