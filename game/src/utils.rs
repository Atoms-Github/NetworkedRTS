use std::any::TypeId;
use crossbeam_channel::Receiver;



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
