use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::rts::game::game_state::GameResources;
use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use ggez::event::MouseButton;

pub struct InputComp{
    pub mode: InputMode,
}
#[derive(Clone)]
pub enum InputMode{
    None,
    SelectionBox(GlobalEntityID),
    ClickUI(GlobalEntityID),
    PanCamera,
}

pub static INPUT_SYS: System<GameResources> = System{
    run
};
fn run(res: &GameResources, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (player_id, player, camera, input) in CompIter3::<PlayerComp, CameraComp, InputComp>::new(c){
        match input.mode.clone(){
            InputMode::None => {
                if player.inputs.mouse_event == RtsMouseEvent::MouseDown(MouseButton::Middle){
                    input.mode = InputMode::PanCamera;
                }
            }
            InputMode::SelectionBox(_) => {}
            InputMode::ClickUI(_) => {}
            InputMode::PanCamera => {
                if player.inputs.mouse_event == RtsMouseEvent::MouseUp{
                    input.mode = InputMode::None;
                }
                camera.translation -= &player.inputs.mouse_moved;

            }
        }
    }
}


