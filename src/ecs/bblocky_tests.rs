use super::bblocky::*;
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


#[test]
fn test_ser_de() {
    let original = TestStructB{
        integer: 3,
        vec: vec![vec![], vec![TestStructA{
            integer: 0,
            float: 3.7,
            vec: vec![8,5]
        }]],
        float: 100.2
    };
    let strb = SuperAny::new(original.clone());
    let bytes = bincode::serialize(&strb).unwrap();
    let reser = bincode::deserialize::<SuperAny>(bytes.as_slice()).unwrap();

    let new_version = reser.get::<TestStructB>().clone();
    assert_eq!(original, new_version);
}

#[test]
fn test_clone() {
    let original = TestStructB{
        integer: 3,
        vec: vec![vec![], vec![TestStructA{
            integer: 0,
            float: 3.7,
            vec: vec![8,5]
        }]],
        float: 100.2
    };
    let super_any = SuperAny::new(original.clone());
    assert_eq!(*super_any.clone().get::<TestStructB>(), original);
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TestStructA{
    integer: u32,
    float: f32,
    vec: Vec<i32>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TestStructB{
    integer: u32,
    vec: Vec<Vec<TestStructA>>,
    float: f32,
}
