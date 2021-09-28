use crate::rts::game::game_state::*;
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
    pub selected_map: String,
}

pub static LOBBY_SYS: System = System{
    run,
    name: "lobby"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    let scene = c.find_scene();
    for (lobby_id, lobby) in CompIter1::<LobbyManager>::new(c){
        lobby.game_start_cooldown -= crate::netcode::common::time::timekeeping::FRAME_DURATION_MILLIS;
        // Check for game start on F1.
        for (player_id , input, player) in CompIter2::<InputComp, PlayerComp>::new(c) {
            if input.inputs.primitive.is_keycode_pressed(VirtualKeyCode::F1){
                scene.next = SceneType::InGame;
            }
        }
    }

}






