use std::ops::Div;

use ggez::event::MouseButton;

use netcode::common::net_game_state::StaticFrameData;

use crate::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CameraComp{
    pub translation: PointFloat,
    pub zoom: f32,
}

impl CameraComp{
    pub fn get_as_screen_transform(&self, ecs: &CompStorage, entity_id: GlobalEntityID) -> (PointFloat, PointFloat){
        let position_comp  = ecs.get::<PositionComp>(entity_id).unwrap();
        let size_comp  = &ecs.get::<SizeComp>(entity_id).unwrap();

        if let Some(ui_comp) = ecs.get::<UIComp>(entity_id){
            return (position_comp.pos.clone(), size_comp.size.clone());
        }else{
            let pos_screen = self.game_space_to_screen_space(position_comp.pos.clone());
            let size_screen = self.game_size_to_screen_size(size_comp.size.clone());
            return (pos_screen, size_screen);
        }
    }
    pub fn game_space_to_screen_space(&self, coords: PointFloat) -> PointFloat{
        return (coords - &self.translation) * self.zoom;
    }
    pub fn screen_space_to_game_space(&self, coords: PointFloat) -> PointFloat{
        return coords / self.zoom + &self.translation;
    }
    pub fn game_size_to_screen_size(&self, coords: PointFloat) -> PointFloat{
        return coords * self.zoom;
    }
}

pub static CAMERA_PAN_SYS: System = System{
    run,
    name: "camera_pan"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    for (player_id, camera, input) in CompIter2::<CameraComp, InputComp>::new(c){
        if input.inputs.mouse_event == NiceMouseEvent::MouseDown(MouseButton::Middle){
            input.is_panning = true;
        }
        if input.is_panning{
            if input.inputs.mouse_event == NiceMouseEvent::MouseUp{
                input.is_panning = false;
            }
            camera.translation -= input.inputs.mouse_moved.clone().div(camera.zoom);
        }

        let amount = 1.2;
        let zoom_move = 0.2;
        if input.inputs.mouse_scrolled > 0.0 && camera.zoom  < 2.0{
            camera.zoom *= amount;
            let mouse_game_new =  camera.screen_space_to_game_space(input.inputs.primitive.get_mouse_loc().clone());
            let mouse_xy = mouse_game_new - &camera.translation;
            camera.translation += mouse_xy * zoom_move;
        }else if input.inputs.mouse_scrolled < 0.0 && camera.zoom > 0.5{
            let mouse_game_new =  camera.screen_space_to_game_space(input.inputs.primitive.get_mouse_loc().clone());
            camera.zoom /= amount;
            let mouse_xy = mouse_game_new - &camera.translation;
            camera.translation -= mouse_xy * zoom_move;
        }

    }
}

