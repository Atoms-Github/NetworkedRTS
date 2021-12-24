use std::collections::{BTreeMap, BTreeSet};
use crate::utils::{TypeIdNum, gett};
use serde::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::ecs::comp_store::TypesSet;
use super::comp_store::SingleComp;
use crate::ecs::bblocky::super_any::SuperAny;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PendingEntity<C>{
    data: BTreeMap<TypeIdNum, SingleComp<C>>,
}

impl<C> PendingEntity<C> {
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
    pub fn iter(&self) -> std::collections::btree_map::Iter<TypeIdNum, SingleComp<C>>{ // Optimum make it return move instead of reference (then clone).
        return self.data.iter();
    }
    pub fn add_comp<T: 'static + Send>(&mut self, value: T) {
        assert!(self.set_comp(value).is_none(), "Pending entity already contained that component type!");
    }
    pub fn set_comp<T: 'static + Send>(&mut self, value: T) -> Option<SingleComp<C>> {
        let bytes = unsafe {crate::unsafe_utils::struct_as_u8_slice(&value)}.to_vec();
        return self.data.insert(crate::utils::gett::<T>(), SuperAny::new(value));
    }
    pub fn remove<T: 'static + Send>(&mut self) {
        self.data.remove(&crate::utils::gett::<T>());
    }
    pub fn get_mut<T: 'static + Send>(&mut self) -> Option<&mut T>{
        self.data.get_mut(&gett::<T>()).map(|item|{item.get_mut::<T>()})
    }
}

