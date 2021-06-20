use std::collections::BTreeMap;
use crate::utils::TypeIdNum;
use serde::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::ecs_new_dawn::comp_store::TypesHash;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PendingEntity{
    data: BTreeMap<TypeIdNum, Vec<u8>>,
}

impl PendingEntity {
    pub fn hash_types(&self) -> TypesHash{
        let test = self.data.keys().collect::<Vec<&TypeIdNum>>();
        let mut s = DefaultHasher::new();
        test.hash(&mut s);
        s.finish()
    }
    pub fn iter(&self) -> std::collections::btree_map::Iter<TypeIdNum, Vec<u8>>{ // Optimum make it return move instead of reference (then clone).
        return self.data.iter();
    }
    pub fn insert<T: 'static>(&mut self, value: T) {
        let bytes = unsafe {any_as_u8_slice(&value)}.to_vec();
        let result = self.data.insert(crate::utils::get_type_id::<T>(), bytes);
        if result.is_some(){
            panic!("Pending entity already contained that component type!");
        }
    }
    pub fn remove<T: 'static>(&mut self) {
        self.data.remove(&crate::utils::get_type_id::<T>());
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}