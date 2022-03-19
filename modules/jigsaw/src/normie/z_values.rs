use crate::*;
use netcode::*;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum JZValue {
    Mat,
    StuckInPieces,
    GamePiece = 20_000,
    JigsawPieceHeld = 40_000,
    UI,
}
impl JZValue {
    pub fn g(&self) -> ZType{
        return *self as ZType;
    }
}
