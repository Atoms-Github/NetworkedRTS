use serde::{Serialize, Deserialize};
use std::collections::{HashSet};
use ggez::event::{KeyCode};

use crate::common::types::*;
//use ggez::input::mouse::MouseButton;
use ggez::input::mouse::MouseButton;


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct InputState {
    pub mouse_loc: nalgebra::Point2<f32>,
    pub keys_pressed: HashSet<usize>, // Size = 260ish. Would use array but serialization is a bit weird.
    pub conn_status_update: ConnStatusChangeType,
    pub mouse_btns_pressed: HashSet<MouseButton>
}

impl Eq for InputState{

}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConnStatusChangeType {
    Nothing,
    Connecting,
    Disconnecting,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum InputChange {
    KeyDownUp(KeyCode, bool),
    NewMousePosition(f32, f32),
    MouseUpDown(MouseButton, bool),
    MouseMove(PointFloat)
}

impl InputChange{ // TODO2: Swap round. Should be state.apply_change(InputChange);
    pub fn apply_to_state(&self, state: &mut InputState){
        match self{
            InputChange::KeyDownUp(code, is_pressed) => {
                state.set_keycode_pressed(*code, *is_pressed);
            },
            InputChange::MouseMove(position) => {
                state.mouse_loc = position.clone();
            }
            InputChange::NewMousePosition(x, y) => {
                state.mouse_loc = PointFloat::new(*x,*y);
            }
            InputChange::MouseUpDown(button, is_down) => {
                if *is_down{
                    state.mouse_btns_pressed.insert(*button);
                }else{
                    state.mouse_btns_pressed.remove(button);
                }
            }
        }
    }
}
impl Default for InputState{
    fn default() -> Self {
        Self::new()
    }
}

impl InputState{
    pub fn new() -> InputState{
        InputState{
            mouse_loc: PointFloat::new(0.0, 0.0),
            keys_pressed: HashSet::new(),
//            keys_pressed: vec![false; 260]
            conn_status_update: ConnStatusChangeType::Nothing,
            mouse_btns_pressed: Default::default()
        }
    }
    pub fn is_keyid_pressed(&self, key_id: usize) -> bool{
        self.keys_pressed.contains(&key_id)
    }
    pub fn is_keycode_pressed(&self, code: KeyCode) -> bool{
        self.is_keyid_pressed(code as usize)
    }
    pub fn set_keyid_pressed(&mut self, key_id: usize, pressed: bool){
        if pressed{
            self.keys_pressed.insert(key_id);
        }else{
            self.keys_pressed.remove(&key_id);
        }
    }
    pub fn set_keycode_pressed(&mut self, code: KeyCode, pressed: bool){
        self.set_keyid_pressed(code as usize, pressed)
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
        if self.is_keycode_pressed(KeyCode::A) || self.is_keycode_pressed(KeyCode::Left) {
            x -= 1.0;
        }

        (x,y)
    }
}







