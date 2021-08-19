use serde::{Serialize, Deserialize};
use std::collections::{HashSet};
use ggez::event::{KeyCode};

//use ggez::input::mouse::MouseButton;
use ggez::input::mouse::MouseButton;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
use serde_big_array::*;
use nalgebra::{VectorN, U2};

// big_array! { BigArray; }

const MOUSE_BUTTONS_COUNT: usize = 24;
const KEY_COUNT : usize = 192;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct InputState {
    mouse_loc: PointFloat,
    #[serde(with = "BigArray")]
    keys_pressed: [bool; KEY_COUNT],
    mouse_btns_pressed: [bool; MOUSE_BUTTONS_COUNT] // NOT SUPPORTING NON-DEFAULT MOUSE BUTTONS.
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
                state.set_mouse_pressed(*button, *is_down);
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
    pub fn get_keys_array(&self) -> &[bool; KEY_COUNT]{
        return &self.keys_pressed;
    }
    pub fn get_mouse_array(&self) -> &[bool; MOUSE_BUTTONS_COUNT]{
        return &self.mouse_btns_pressed;
    }
    pub fn new() -> InputState{
        InputState{
            mouse_loc: PointFloat::new(0.0, 0.0),
            keys_pressed: [false; KEY_COUNT],
            mouse_btns_pressed: [false; MOUSE_BUTTONS_COUNT],
        }
    }
    pub fn u32_to_keycode(num: u32) -> Option<KeyCode>{
        if num >= KeyCode::Key1 as u32 && num <= KeyCode::Cut as u32 {
            Some(unsafe { std::mem::transmute(num) })
        } else {
            None
        }
    }
    pub fn get_mouse_loc(&self) -> &PointFloat{
        return &self.mouse_loc;
    }
    pub fn get_button_index(button: MouseButton) -> usize{
        match button{
            MouseButton::Left => {0}
            MouseButton::Right => {1}
            MouseButton::Middle => {2}
            MouseButton::Other(bonus) => {
                assert!(bonus <= 20, "Invalid mouse bonus button! Only up to 20 are supported. {}", bonus);
                (3 + bonus).into()
            }
        }
    }
    pub fn is_modif_key(key: u32) -> bool{
        let id = InputState::u32_to_keycode(key).unwrap();
        return id == KeyCode::LShift || id == KeyCode::LAlt || id == KeyCode::LWin || id == KeyCode::LControl;
    }
    pub fn get_button_enum(button: usize) -> MouseButton{
        match button{
            0 => MouseButton::Left,
            1 => MouseButton::Right,
            2 => MouseButton::Middle,
            other => {
                let bonus = other - 3;
                MouseButton::Other(bonus as u8)
            }
        }
    }
    pub fn set_mouse_pressed(&mut self, button: MouseButton, is_down: bool){
        let index = Self::get_button_index(button);
        self.mouse_btns_pressed[index] = is_down;
    }
    pub fn get_mouse_pressed(&self, button: MouseButton) -> bool{
        let index = Self::get_button_index(button);
        self.mouse_btns_pressed[index]
    }
    pub fn is_keyid_pressed(&self, key_id: usize) -> bool{
        self.keys_pressed[key_id]
    }
    pub fn is_keycode_pressed(&self, code: KeyCode) -> bool{
        self.is_keyid_pressed(code as usize)
    }
    pub fn set_keyid_pressed(&mut self, key_id: usize, pressed: bool){
        self.keys_pressed[key_id] = pressed;
    }
    pub fn set_keycode_pressed(&mut self, code: KeyCode, pressed: bool){
        self.set_keyid_pressed(code as usize, pressed)
    }

    pub fn get_directional(&self) -> VectorN<f32, U2>{
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
        PointFloat::new(x,y)
    }
}







