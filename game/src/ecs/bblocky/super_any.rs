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
use crate::ecs::bblocky::super_vec::SuperVec;
use serde::*;

use crate::rts::compsys::*;

use crate::utils::*;
use crate::ecs::comp_store::*;
use super::comp_registration::*;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuperAny<C> {
    pub list: SuperVec<C>,
}
impl<C> SuperAny<C> {
    pub fn new<T : 'static + Send>(item: T) -> Self{
        Self{
            list: SuperVec::new_and_push(item),
        }
    }
    pub fn get<T : 'static>(&self) -> &T{
        return self.list.get::<T>(0).unwrap();
    }
    pub fn get_mut<T : 'static>(&mut self) -> &mut T{
        return self.list.get_mut::<T>(0).unwrap();
    }
}