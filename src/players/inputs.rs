use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use ggez::event::KeyCode;


type PointFloat = nalgebra::Point2<f32>;




#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputState {
    pub mouse_loc: nalgebra::Point2<f32>,
    pub keys_pressed: Vec<bool>, // Size = 260ish. Would use array but serialization is a bit weird. // TODO figure out how array serialization works.
}

impl InputState{
    pub fn new() -> InputState{
        InputState{
            mouse_loc: PointFloat::new(0.0, 0.0),
            keys_pressed: vec![false; 500],
        }
    }
}