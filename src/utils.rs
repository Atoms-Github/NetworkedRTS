use std::any::TypeId;

pub type TypeIdNum = u64;

struct CrackerTypeId {
    t: TypeIdNum,
}

pub fn break_open_type_id(type_id: TypeId) -> TypeIdNum{
    let emp_exposed: CrackerTypeId = unsafe {
        std::mem::transmute(type_id)
    };
    return emp_exposed.t;
}
pub fn crack_type_id<T : 'static>() -> TypeIdNum{
    break_open_type_id(TypeId::of::<T>())
}