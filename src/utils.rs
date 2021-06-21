use std::any::TypeId;
use crate::rts::comps::player_comp::PLAYER_NAME_SIZE_MAX;

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
pub fn gett<T : 'static>() -> TypeIdNum{
    break_open_type_id(TypeId::of::<T>())
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