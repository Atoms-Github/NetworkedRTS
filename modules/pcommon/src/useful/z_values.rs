use crate::*;
use netcode::*;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum CZValue {
    Cursor = 60_000,
}
impl CZValue {
    pub fn g(&self) -> ZType{
        return *self as ZType;
    }
}
