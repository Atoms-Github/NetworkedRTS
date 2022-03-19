use crate::*;
use ggez::event::{KeyCode, MouseButton};

use netcode::ServerEvent;

use netcode::common::input_state::InputState;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InputComp{
    pub is_panning: bool,
    pub inputs: NiceInputState,
    pub hovered_entity: Option<GlobalEntityID>,
    pub mouse_pos_game_world: PointFloat,
}
impl Default for InputComp{
    fn default() -> Self {
        Self{
            is_panning: false,
            inputs: Default::default(),
            hovered_entity: None,
            mouse_pos_game_world: PointFloat::new(0.0, 0.0),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct NiceInputState { // A more processed version of the 'primative' input state.
    pub primitive: InputState,
    pub mouse_event: NiceMouseEvent,
    pub key_event: NiceKeyEvent,
    pub mouse_moved: PointFloat,
    pub mouse_scrolled: f32,
    mouse_btn_held: Option<usize>,
    key_held: Option<usize>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum NiceMouseEvent {
    MouseDown(MouseButton),
    MouseUp,
    NoMouse,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum NiceKeyEvent {
    KeyDown(KeyCode),
    KeyUp,
    NoKey,
}
impl NiceInputState {
    pub fn update_input_state(&mut self, new_input: InputState){
        self.mouse_moved = new_input.get_mouse_loc().clone() - self.primitive.get_mouse_loc();
        self.mouse_scrolled = new_input.total_scroll_dist - self.primitive.total_scroll_dist;

        self.mouse_event = NiceMouseEvent::NoMouse;
        self.key_event = NiceKeyEvent::NoKey;

        match self.mouse_btn_held {
            None => {
                for (mouse_id, is_down) in new_input.get_mouse_array().iter().enumerate(){
                    if *is_down{
                        self.mouse_event = NiceMouseEvent::MouseDown(InputState::get_button_enum(mouse_id));
                        self.mouse_btn_held = Some(mouse_id);
                        break;
                    }
                }
            }
            Some(mouse_button) => {
                if new_input.get_mouse_array()[mouse_button] == false{
                    self.mouse_event = NiceMouseEvent::MouseUp;
                    self.mouse_btn_held = None;
                }
            }
        }
        match self.key_held {
            None => {
                for (key_id, is_down) in new_input.get_keys_array().iter().enumerate(){
                    if *is_down && !InputState::is_modif_key(key_id as u32){
                        self.key_event = NiceKeyEvent::KeyDown(InputState::u32_to_keycode(key_id as u32).unwrap());
                        self.key_held = Some(key_id);
                        break;
                    }
                }
            }
            Some(key) => {
                if new_input.get_keys_array()[key] == false{
                    self.key_event = NiceKeyEvent::KeyUp;
                    self.key_held = None;
                }
            }
        }


        self.primitive = new_input;
    }
}
impl Default for NiceInputState {
    fn default() -> Self {
        Self{
            primitive: InputState::default(),
            mouse_event: NiceMouseEvent::NoMouse,
            key_event: NiceKeyEvent::NoKey,
            mouse_moved: PointFloat::new(0.0,0.0),
            mouse_scrolled: 0.0,
            mouse_btn_held: None,
            key_held: None
        }

    }
}
