use std::any::TypeId;
use crate::rts::compsys::player::PLAYER_NAME_SIZE_MAX;

pub type TypeIdNum = u64;

struct CrackerTypeId {
    t: TypeIdNum,
}

pub fn crack_type_id(type_id: TypeId) -> TypeIdNum{
    let emp_exposed: CrackerTypeId = unsafe {
        std::mem::transmute(type_id)
    };
    return emp_exposed.t;
}
pub fn gett<T : 'static>() -> TypeIdNum{
    crack_type_id(TypeId::of::<T>())
}


pub fn get_line_input(message: &str) -> String{
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    println!("{}", message);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s;
}


pub fn pad_name(name: String) -> [u8; PLAYER_NAME_SIZE_MAX]{
    let mut buffer = [0;PLAYER_NAME_SIZE_MAX];
    let name_bytes = name.as_bytes();

    buffer[..name_bytes.len()].clone_from_slice(&name_bytes);
    return buffer;
}

pub unsafe fn unsafe_const_cheat<T>(reference: &T) -> &mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}