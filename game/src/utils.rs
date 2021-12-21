use std::any::TypeId;
use crossbeam_channel::Receiver;

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


// pub fn pad_name(name: String) -> [u8; PLAYER_NAME_SIZE_MAX]{
//     let mut buffer = [0;PLAYER_NAME_SIZE_MAX];
//     let name_bytes = name.as_bytes();
// 
//     buffer[..name_bytes.len()].clone_from_slice(&name_bytes);
//     return buffer;
// }

pub unsafe fn unsafe_const_cheat<T>(reference: &T) -> &mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}



pub fn compress(uncompressed: Vec<u8>) -> Vec<u8>{
    let mut encoder = snap::Encoder::new();

    let mut compressed = encoder.compress_vec(&uncompressed).unwrap();

    println!("Compressed {} to {}", uncompressed.len(), compressed.len());
    return compressed;
}
pub fn decompress(compressed: Vec<u8>) -> Vec<u8>{
    let mut decoder = snap::Decoder::new();
    let uncompressed = decoder.decompress_vec(&compressed).unwrap();
    println!("Decompressed {} from {}", uncompressed.len(), compressed.len());
    return uncompressed;
}
#[macro_export]
macro_rules! unwrap {
        ($enum:path, $expr:expr) => {{
            if let $enum(item) = $expr {
                item
            } else {
                panic!("Wrong match type!!")
            }
        }};
    }

pub use unwrap;