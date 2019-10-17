


use std::collections::HashMap;

use crate::players::inputs::InputState;
use crate::ecs::world::*;

pub type PlayerID = i16;
pub type FrameIndex = i16;



pub struct GameState{
    pub world: World,
    pub storages: Storages,
    pub frame_count: i32,
}

pub struct InputsFrame{
    pub inputs: HashMap<PlayerID, InputState>
}


impl GameState{
    pub fn new() -> GameState{
        GameState{
            world: World::new(),
            storages: Storages::new(),
            frame_count: 0
        }
    }

    pub fn simulate_tick(inputs_info: &InputsFrame){

    }
}