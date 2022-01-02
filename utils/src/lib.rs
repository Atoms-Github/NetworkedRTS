#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(core_intrinsics)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_unsafe)] // TODO2: Investigate the need for this.
#![feature(drain_filter)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(deprecated)] // TODO:

pub mod debug_timer;

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
