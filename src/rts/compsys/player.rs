
use crate::netcode::InputState;

pub const PLAYER_NAME_SIZE_MAX: usize = 12;

pub struct PlayerComp {
    pub inputs: InputState,
    pub name: [u8; PLAYER_NAME_SIZE_MAX]
}