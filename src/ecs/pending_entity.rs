use std::collections::{BTreeMap, BTreeSet};
use crate::utils::TypeIdNum;
use serde::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::ecs::comp_store::TypesSet;
use super::comp_store::SingleComp;
use crate::ecs::bblocky::SuperAny;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PendingEntity{
    data: BTreeMap<TypeIdNum, SingleComp>,
}

impl PendingEntity {
    pub fn new() -> Self{
        Self::default()
    }
    pub fn hash_types(&self) -> TypesSet {
        let mut types = BTreeSet::new();
        for new_type in self.data.keys(){
            types.insert(*new_type);
        }
        return types;
    }
    pub fn iter(&self) -> std::collections::btree_map::Iter<TypeIdNum, SingleComp>{ // Optimum make it return move instead of reference (then clone).
        return self.data.iter();
    }
    pub fn add_comp<T: 'static + Send>(&mut self, value: T) {
        assert!(self.set_comp(value).is_none(), "Pending entity already contained that component type!");
    }
    pub fn set_comp<T: 'static + Send>(&mut self, value: T) -> Option<SingleComp> {
        let bytes = unsafe {any_as_u8_slice(&value)}.to_vec();
        return self.data.insert(crate::utils::gett::<T>(), SuperAny::new(value));
    }
    pub fn remove<T: 'static + Send>(&mut self) {
        self.data.remove(&crate::utils::gett::<T>());
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}