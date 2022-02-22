use crate::*;
use netcode::*;
use game::pub_types::ZType;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum ZValue {
    Arena,
    ArenaBoxes,
    BelowGamePiece,
    InGameUIBelow,
    GamePiece = 20_000,
    JigsawPieceHeld = 40_000,
    InGameUI,
    SelectionBox,
    UI,
    AboveUI,
    Cursor,
}
impl ZValue {
    pub fn g(&self) -> ZType{
        return *self as ZType;
    }
}
