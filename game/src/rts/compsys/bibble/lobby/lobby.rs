use crate::rts::compsys::jigsaw::jigsaw_game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::event::MouseButton;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
use std::ops::Mul;
use mopa::Any;
use std::ops::Div;
use crate::bibble::effect_resolver::revolver::Revolver;
use crate::bibble::data::data_types::{RaceID, VirtualKeyCode};


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LobbyManager{ // Means keep when scene changes.
    pub game_start_cooldown: f32,
}

pub fn lobby_sys<C>() -> System<C>{
    System{
        run,
        name: "lobby"
    }
}
fn run<C>(c: &mut CompStorage<C>, ent_changes: &mut EntStructureChanges<C>, meta: &SimMetadata){
    let scene = c.find_scene();
    for (lobby_id, lobby) in CompIter1::<LobbyManager>::new(c){
        lobby.game_start_cooldown -= meta.delta;
        // Check for game start on F1.
        for (player_id , input, player) in CompIter2::<InputComp, PlayerComp>::new(c) {
            if input.inputs.primitive.is_keycode_pressed(VirtualKeyCode::F1){
                // scene.next = SceneType::InGame;
            }else if input.inputs.primitive.is_keycode_pressed(VirtualKeyCode::F2){
                scene.next = SceneType::InJigsaw;
            }
        }
    }

}






