use super::super_vec::*;
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
use crate::ecs::bblocky::super_any::SuperAny;
use crate::ecs::bblocky::super_any_tests::*;


#[test]
fn test_ser_de() {
    let original = TestStructC{
        byte_a: 3,
        byte_b: 2,
        byte_c: 5
    };
    let my_type = gett::<TestStructC>();

    let super_any = SuperAny::new(original.clone());
    let mut my_vec = SuperVec::new(my_type);

    my_vec.push(super_any);

    let bytes = bincode::serialize(&my_vec).unwrap();
    let reser = bincode::deserialize::<SuperVec>(bytes.as_slice()).unwrap();

    let new_version = reser.get::<TestStructC>(0).unwrap().clone();
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