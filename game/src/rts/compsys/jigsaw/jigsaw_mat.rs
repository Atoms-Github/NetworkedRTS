
use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use std::ops::Div;

use ggez::event::MouseButton;
use crate::bibble::data::data_types::VirtualKeyCode;
use rand::Rng;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct JigsawMatComp{
    pub jigsaw_name: String,
    pub next_jigsaw_z: ZType,
}


pub fn jigsaw_mat_sys<C>() -> System<C>{
    System{
        run,
        name: "jigsaw_mat"
    }
}
fn run<C>(c: &mut CompStorage<C>, ent_changes: &mut EntStructureChanges<C>, meta: &SimMetadata){
    let scene = c.find_scene();
    if let Some(mat_comp) = c.find_jigsaw_mat(){
        // Check for jigsaw end on F3.
        for (player_id , input, player) in CompIter2::<InputComp, PlayerComp>::new(c) {
            if input.inputs.primitive.is_keycode_pressed(VirtualKeyCode::F3){
                scene.next = SceneType::Lobby;
            }
        }
    }
}



