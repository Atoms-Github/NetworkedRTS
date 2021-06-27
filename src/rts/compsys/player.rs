
use crate::netcode::InputState;
use ggez::event::{KeyCode, MouseButton};
use crate::rts::compsys::RtsMouseEvent::{NoMouse, MouseUp};
use crate::rts::compsys::RtsKeyEvent::NoKey;

pub const PLAYER_NAME_SIZE_MAX: usize = 12;

pub struct PlayerComp {
    pub rts_inputs: RtsInputState,
    pub name: [u8; PLAYER_NAME_SIZE_MAX]
}

pub struct RtsInputState{
    pub primitive: InputState,
    pub mouse_event: RtsMouseEvent,
    pub key_event: RtsKeyEvent,
    mouse_btn_held: Option<usize>,
    key_held: Option<usize>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum RtsMouseEvent {
    MouseDown(MouseButton),
    MouseUp,
    NoMouse,
}
#[derive(Clone, Debug, PartialEq)]
pub enum RtsKeyEvent {
    KeyDown(KeyCode),
    KeyUp,
    NoKey,
}
impl RtsInputState{
    pub fn set_input_state(&mut self, new_input: InputState){
        self.mouse_event = NoMouse;
        self.key_event = NoKey;

        match self.mouse_btn_held.clone() {
            None => {
                for (mouse_id, is_down) in new_input.get_mouse_array().iter().enumerate(){
                    if *is_down{
                        self.mouse_event = RtsMouseEvent::MouseDown(InputState::get_button_enum(mouse_id));
                        self.mouse_btn_held = Some(mouse_id);
                        break;
                    }
                }
            }
            Some(mouse_button) => {
                if new_input.get_mouse_array()[mouse_button] == false{
                    self.mouse_event = RtsMouseEvent::MouseUp;
                    self.mouse_btn_held = None;
                }
            }
        }
        match self.key_held {
            None => {
                for (key_id, is_down) in new_input.get_keys_array().iter().enumerate(){
                    if *is_down && !InputState::is_modif_key(key_id as u32){
                        self.key_event = RtsKeyEvent::KeyDown(InputState::u32_to_keycode(key_id as u32).unwrap());
                        self.key_held = Some(key_id);
                        break;
                    }
                }
            }
            Some(key) => {
                if new_input.get_keys_array()[key] == false{
                    self.key_event = RtsKeyEvent::KeyUp;
                    self.key_held = None;
                }
            }
        }

        self.primitive = new_input;
    }
}
impl Default for RtsInputState{
    fn default() -> Self {
        Self{
            primitive: InputState::default(),
            mouse_event: RtsMouseEvent::NoMouse,
            key_event: RtsKeyEvent::NoKey,
            mouse_btn_held: None,
            key_held: None
        }

    }
}

/*
    pub fn register_mouse_click_event(&mut self, button: MouseButton, is_down: bool){
        self.active_snapshot.mouse_btns_pressed[get_button_index(button)] = is_down;
        if is_down && self.waiting_on_mouse.is_none(){
            self.waiting_on_mouse = Some(button);
            self.active_snapshot.events.push(InputEvent::MouseDown(button));
        }else if !is_down && self.waiting_on_mouse == Some(button){
            self.waiting_on_mouse = None;
            self.active_snapshot.events.push(InputEvent::MouseUp());
        }
    }
    pub fn register_key_event(&mut self, key: KeyCode, is_down: bool, repeat: bool){
        self.active_snapshot.keys_pressed[key as usize] = is_down;
        if is_down && self.waiting_on_key.is_none(){
            self.waiting_on_key = Some(key);
            self.active_snapshot.events.push(InputEvent::KeyDown(key, repeat));
        }else if !is_down && self.waiting_on_key == Some(key){
            self.waiting_on_key = None;
            self.active_snapshot.events.push(InputEvent::KeyUp());
        }
    }
 */