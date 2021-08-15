use ggez::event::{KeyCode, MouseButton};

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::netcode::InputState;
use crate::pub_types::PointFloat;
use crate::rts::compsys::*;
use crate::rts::game::game_state::GameResources;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InputComp{
    pub is_panning: bool,
    pub mode: InputMode, // TODO: Add boolean here for 'isPanning', so can pan while units selected.
    pub inputs: RtsInputState,
    pub hovered_entity: Option<GlobalEntityID>,
    pub mouse_pos_game_world: PointFloat,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum InputMode{
    None,
    SelectionBox,
    UnitsSelected, // TODO: Remove?
    ClickUI(GlobalEntityID),
    TargettingAbility,

}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RtsInputState{
    pub primitive: InputState,
    pub mouse_event: RtsMouseEvent,
    pub key_event: RtsKeyEvent,
    pub mouse_moved: PointFloat,
    mouse_btn_held: Option<usize>,
    key_held: Option<usize>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum RtsMouseEvent {
    MouseDown(MouseButton),
    MouseUp,
    NoMouse,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum RtsKeyEvent {
    KeyDown(KeyCode),
    KeyUp,
    NoKey,
}
impl RtsInputState{
    pub fn set_input_state(&mut self, new_input: InputState){
        self.mouse_moved = new_input.get_mouse_loc().clone() - self.primitive.get_mouse_loc();

        self.mouse_event = RtsMouseEvent::NoMouse;
        self.key_event = RtsKeyEvent::NoKey;

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
            mouse_moved: PointFloat::new(0.0,0.0),
            mouse_btn_held: None,
            key_held: None
        }

    }
}
