use std::any::TypeId;

pub type TypeIdNum = u64;

struct CrackerTypeId {
    t: TypeIdNum,
}
pub trait MyReplaceTrait<T>{
    fn my_replace(&mut self, index: usize, new_item: T);
}
impl<T> MyReplaceTrait<T> for Vec<T>{
    fn my_replace(&mut self, index: usize, new_item: T){
        std::mem::replace(&mut self[index], new_item);
    }
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