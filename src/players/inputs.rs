use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use ggez::event::KeyCode;
use ggez::input::keyboard::is_key_pressed;


type PointFloat = nalgebra::Point2<f32>;

use std::time::{SystemTime, UNIX_EPOCH};




#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputState {
    pub mouse_loc: nalgebra::Point2<f32>,
    pub keys_pressed: HashSet<usize>, // Size = 260ish. Would use array but serialization is a bit weird. // TODO figure out how array serialization works.
}

pub enum InputChange {
    KeyDownUp(KeyCode, bool),
    MouseMove(PointFloat)
}


impl InputState{
    pub fn new() -> InputState{
        InputState{
            mouse_loc: PointFloat::new(0.0, 0.0),
            keys_pressed: HashSet::new(),
//            keys_pressed: vec![false; 260], // TODO find real limit of keys_pressed array.
        }
    }
    pub fn is_keyid_pressed(&self, key_id: usize) -> bool{
        return self.keys_pressed.contains(&key_id);
    }
    pub fn is_keycode_pressed(&self, code: KeyCode) -> bool{
        return self.is_keyid_pressed(code as usize);
    }
    pub fn set_keyid_pressed(&mut self, key_id: usize, pressed: bool){
        if pressed{
            self.keys_pressed.insert(key_id);
        }else{
            self.keys_pressed.remove(&key_id);
        }
    }
    pub fn set_keycode_pressed(&mut self, code: KeyCode, pressed: bool){
        return self.set_keyid_pressed(code as usize, pressed);
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








