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

pub(crate) use serde::de::DeserializeOwned;
pub(crate) use serde::Deserialize;
pub(crate) use serde::Serialize;
pub(crate) use becs::*;
pub(crate) mod pub_types;
pub(crate) mod useful;

pub use compsys::*;
pub use useful::*;
pub use pub_types::*;

pub mod compsys;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
