use winit::event::MouseButton;

use netcode::common::net_game_state::StaticFrameData;

use crate::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ClickableComp {
    pub clicking_on: Option<GlobalEntityID>
}

pub static BUTTON_SYS: System = System{
    run,
    name: "button"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    // Clear all clicking ons.
    for (button_id, button) in CompIter1::<ClickableComp>::new(c){
        button.clicking_on = None;
    }
    for (player_id, input) in CompIter1::<InputComp>::new(c){
        if input.inputs.mouse_event == NiceMouseEvent::MouseDown(MouseButton::Left){
            if let Some(hovered_ent) = input.hovered_entity{
                if let Some(button_comp) = c.get_mut::<ClickableComp>(hovered_ent){
                    button_comp.clicking_on = Some(player_id);
                }
            }
        }
    }
}

