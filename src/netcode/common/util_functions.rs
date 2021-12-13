

// I know util functions are sins, but still ....

use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

pub fn vec_replace_or_end<T>(vec: &mut Vec<T>, insertion_index: usize, item: T){
    if insertion_index < vec.len(){
        vec.remove(insertion_index);
        vec.insert(insertion_index, item);
    }else if insertion_index == vec.len(){
        vec.push(item);
    }else{
        panic!("Adding items more than one past the end is disallowed!");
    }
}

trait Flatten<T> {
    fn flatten(self) -> Option<T>;
}
impl<T> Flatten<T> for Option<Option<T>> {
    fn flatten(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v,
        }
    }
}

pub fn gen_fake_address() -> SocketAddr{
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(7,1,2,3),251))
}