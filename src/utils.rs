use std::any::TypeId;

pub type TypeIdNum = u64;

struct CrackerTypeId {
    pub t: TypeIdNum,
}

pub fn crack_type_id(type_id: TypeId) -> TypeIdNum{
    let emp_exposed: CrackerTypeId = unsafe {
        std::mem::transmute(type_id)
    };
    return emp_exposed.t;
}