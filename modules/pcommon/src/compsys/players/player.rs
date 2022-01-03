use crate::*;

use ggez::event::{KeyCode, MouseButton};

use ggez::graphics::Color;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PlayerComp {
    pub name: String,
    pub connected: bool,
    pub color: Shade,
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