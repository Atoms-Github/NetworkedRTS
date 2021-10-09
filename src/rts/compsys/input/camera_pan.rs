use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};

use ggez::event::MouseButton;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CameraComp{
    pub translation: PointFloat,
    pub zoom: f32,
}

impl CameraComp{
    pub fn get_as_screen_coords(&self, ecs: &CompStorage, entity_id: GlobalEntityID) -> (PointFloat, PointFloat){
        let position_comp  = ecs.get::<PositionComp>(entity_id).unwrap();
        let size_comp  = &ecs.get::<SizeComp>(entity_id).unwrap();

        if let Some(ui_comp) = ecs.get::<UIComp>(entity_id){
            return (position_comp.pos.clone(), size_comp.size.clone());
        }else{
            let pos_game = size_comp.get_corner_top_left(position_comp);

            let pos_screen = self.game_space_to_screen_space(pos_game);
            let size_screen = self.game_size_to_screen_size(size_comp.size.clone());
            return (pos_screen, size_screen);
        }


    }
    pub fn game_space_to_screen_space(&self, coords: PointFloat) -> PointFloat{
        return coords - &self.translation;
    }
    pub fn screen_space_to_game_space(&self, coords: PointFloat) -> PointFloat{
        return coords + &self.translation;
    }
    pub fn game_size_to_screen_size(&self, coords: PointFloat) -> PointFloat{
        return coords * self.zoom;
    }
}

pub static CAMERA_PAN_SYS: System = System{
    run,
    name: "camera_pan"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    for (player_id, camera, input) in CompIter2::<CameraComp, InputComp>::new(c){
        if input.inputs.mouse_event == RtsMouseEvent::MouseDown(MouseButton::Middle){
            input.is_panning = true;
        }
        if input.is_panning{
            if input.inputs.mouse_event == RtsMouseEvent::MouseUp{
                input.is_panning = false;
            }
            camera.translation -= &input.inputs.mouse_moved;
        }
    }
}

