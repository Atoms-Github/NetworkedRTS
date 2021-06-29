use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::rts::game::game_state::GameResources;
use ggez::event::MouseButton;

pub struct CameraComp{
    pub translation: PointFloat,
    pub zoom: f32,
}

impl CameraComp{
    pub fn get_as_screen_coords(&self, ecs: &CompStorage, entity_id: GlobalEntityID) -> (PointFloat, PointFloat){

        let position  = &ecs.get::<PositionComp>(entity_id).unwrap().pos.clone();
        let size  = &ecs.get::<SizeComp>(entity_id).unwrap().size.clone();

        let pos_screen = self.game_space_to_screen_space(*position);
        let size_screen = self.game_size_to_screen_size(*size);
        return (pos_screen, size_screen);
    }
    pub fn game_space_to_screen_space(&self, coords: PointFloat) -> PointFloat{
        return coords - &self.translation;
    }
    pub fn game_size_to_screen_size(&self, coords: PointFloat) -> PointFloat{
        return coords * self.zoom;
    }
}

pub static CAMERA_SYS: System<GameResources> = System{
    run
};
fn run(res: &GameResources, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (player_id, camera, input) in CompIter2::<CameraComp, InputComp>::new(c){
        match input.mode.clone(){
            InputMode::None => {
                if input.inputs.mouse_event == RtsMouseEvent::MouseDown(MouseButton::Middle){
                    input.mode = InputMode::PanCamera;
                }
            }
            InputMode::SelectionBox(_) => {}
            InputMode::ClickUI(_) => {}
            InputMode::PanCamera => {
                if input.inputs.mouse_event == RtsMouseEvent::MouseUp{
                    input.mode = InputMode::None;
                }
                camera.translation -= &input.inputs.mouse_moved;

            }
        }
    }
}

