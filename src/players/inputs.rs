use serde::{Serialize, Deserialize};



type PointFloat = nalgebra::Point2<f32>;




#[derive(Clone, Serialize, Deserialize)]
pub struct InputState {
    pub mouse_loc: nalgebra::Point2<f32>,
    pub keys_pressed: Vec<bool>, // Size = 260. Would use array but serialization is a bit weird.

}

impl InputState{
    pub fn new() -> InputState{
        InputState{
            mouse_loc: PointFloat::new(0.0, 0.0),
            keys_pressed: vec![false; 500]
        }
    }
}