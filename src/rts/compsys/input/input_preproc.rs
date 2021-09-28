use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use ggez::event::{MouseButton, KeyCode};
use crate::netcode::InputState;
use crate::pub_types::PointFloat;
use nalgebra::Point2;

pub static INPUT_PREPROC: System = System{
    run,
    name: "input_preproc"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    for (player_id, camera, input) in CompIter2::<CameraComp, InputComp>::new(c){
        input.mouse_pos_game_world = camera.screen_space_to_game_space(input.inputs.primitive.get_mouse_loc().clone());
        input.hovered_entity = None;
        for (ent_id, position, size, render)
        in CompIter3::<PositionComp, SizeComp, RenderComp>::new(c){ // TODO: Could do some sorting.
            let gameworld_rect = size.get_as_rect(position);
            if gameworld_rect.contains(input.mouse_pos_game_world.to_point()){
                input.hovered_entity = Some(ent_id);
                break;
            }
        }
    }
}

