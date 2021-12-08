
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
}


pub static JIGSAW_MAT_SYS: System = System{
    run,
    name: "jigsaw_mat"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    let scene = c.find_scene();
    for (mat_id, mat_comp) in CompIter1::<JigsawMatComp>::new(c){
        // Check for jigsaw end on F3.
        for (player_id , input, player) in CompIter2::<InputComp, PlayerComp>::new(c) {
            if input.inputs.primitive.is_keycode_pressed(VirtualKeyCode::F3){
                scene.next = SceneType::Lobby;
            }
        }
    }
}



