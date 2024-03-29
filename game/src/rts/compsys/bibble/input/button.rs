use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};

use ggez::event::MouseButton;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ClickableComp {
    pub clicking_on: Option<GlobalEntityID>
}

pub fn button_sys<C>() -> System<C>{
    System{
        run,
        name: "button"
    }
}
fn run<C>(c: &mut CompStorage<C>, ent_changes: &mut EntStructureChanges<C>, meta: &SimMetadata){
    // Clear all clicking ons.
    for (button_id, button) in CompIter1::<ClickableComp>::new(c){
        button.clicking_on = None;
    }
    for (player_id, input) in CompIter1::<InputComp>::new(c){
        if input.inputs.mouse_event == RtsMouseEvent::MouseDown(MouseButton::Left)
        && input.mode == InputMode::None{
            if let Some(hovered_ent) = input.hovered_entity{
                if let Some(button_comp) = c.get_mut::<ClickableComp>(hovered_ent){
                    button_comp.clicking_on = Some(player_id);
                }
            }
        }
    }
}

