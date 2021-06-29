

use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::rts::game::game_state::GameResources;
use ggez::event::MouseButton;

pub static INPUT_SELECTION_BOX: System<GameResources> = System{
    run
};
fn run(res: &GameResources, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (player_id, camera, input) in CompIter2::<CameraComp, InputComp>::new(c){
        if input.inputs.mouse_event == RtsMouseEvent::MouseDown(MouseButton::Middle){
            input.mode = InputMode::PanCamera;
        }
        if input.mode == InputMode::PanCamera{
            if input.inputs.mouse_event == RtsMouseEvent::MouseUp{
                input.mode = InputMode::None;
            }
            camera.translation -= &input.inputs.mouse_moved;
        }
    }
}

