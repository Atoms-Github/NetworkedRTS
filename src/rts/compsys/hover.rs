use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::rts::game::game_state::GameResources;
use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use ggez::event::{MouseButton, KeyCode};
use crate::netcode::InputState;
use crate::pub_types::PointFloat;

pub static INPUT_HOVER_SYS: System<GameResources> = System{
    run
};
fn run(res: &GameResources, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (player_id, camera, input) in CompIter2::<CameraComp, InputComp>::new(c){
        // breaking Implement.
    }
}

