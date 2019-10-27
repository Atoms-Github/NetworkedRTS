use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use ggez::input::keyboard::KeyCode;


type PointFloat = nalgebra::Point2<f32>;




#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputState {
    pub mouse_loc: nalgebra::Point2<f32>,
    pub keys_pressed: HashMap<KeyCode, bool>, // Size = 260. Would use array but serialization is a bit weird.

}

impl InputState{
    pub fn new() -> InputState{
        InputState{
            mouse_loc: PointFloat::new(0.0, 0.0),
            keys_pressed: HashMap::new()
        }
    }
}