use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::rts::game::game_state::GameResources;
use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use ggez::event::{MouseButton, KeyCode};
use crate::netcode::InputState;
use crate::pub_types::PointFloat;

pub static INPUT_PREPROC: System<ResourcesPtr> = System{
    run
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (player_id, camera, input) in CompIter2::<CameraComp, InputComp>::new(c){
        // breaking Implement.
        input.mouse_pos_game_world = camera.screen_space_to_game_space(input.inputs.primitive.get_mouse_loc().clone());
    }
}

