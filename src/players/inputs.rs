use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use ggez::event::KeyCode;
use ggez::input::keyboard::is_key_pressed;


type PointFloat = nalgebra::Point2<f32>;

use std::time::{SystemTime, UNIX_EPOCH};


const FRAME_MICROS_DURATION: f64 = 16_000.0; // 16ms


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputState {
    pub mouse_loc: nalgebra::Point2<f32>,
    pub keys_pressed: Vec<bool>, // Size = 260ish. Would use array but serialization is a bit weird. // TODO figure out how array serialization works.
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KnownFrameInfo{
    pub frame_index: usize,
    pub time: SystemTime
}

impl InputState{
    pub fn new() -> InputState{
        InputState{
            mouse_loc: PointFloat::new(0.0, 0.0),
            keys_pressed: vec![false; 500], // TODO find real limit of keys_pressed array.
        }
    }
    pub fn is_keyid_pressed(&self, key_id: usize) -> bool{
        return *self.keys_pressed.get(key_id).expect("Tested is key pressed for index off the end.");
    }
    pub fn is_keycode_pressed(&self, code: KeyCode) -> bool{
        return self.is_keyid_pressed(code as usize);
    }
    pub fn get_directional(&self) -> (f32, f32){
        let mut x = 0.0;
        let mut y = 0.0;

        if self.is_keycode_pressed(KeyCode::W) || self.is_keycode_pressed(KeyCode::Up) {
            y += 1.0;
        }
        if self.is_keycode_pressed(KeyCode::S) || self.is_keycode_pressed(KeyCode::Down) {
            y -= 1.0;
        }
        if self.is_keycode_pressed(KeyCode::D) || self.is_keycode_pressed(KeyCode::Right) {
            x += 1.0;
        }
        if self.is_keycode_pressed(KeyCode::W) || self.is_keycode_pressed(KeyCode::Left) {
            x -= 1.0;
        }

        return (x,y)
    }
}


impl KnownFrameInfo{
    pub fn get_intended_current_frame(&self) -> usize{
        let meme = SystemTime::now().duration_since(self.time).unwrap();

        let dream = self.frame_index + (meme.as_micros() as f64 / FRAME_MICROS_DURATION).floor() as usize;
        return dream;
    }




}









