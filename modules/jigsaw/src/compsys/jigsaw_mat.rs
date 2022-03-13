use crate::*;
use std::ops::Div;

use ggez::event::MouseButton;
use rand::Rng;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct JigsawMatComp{
    pub jigsaw_name: String,
    pub next_jigsaw_z: ZType,
}


pub static JIGSAW_MAT_SYS: System = System{
    run,
    name: "jigsaw_mat"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    let scene = c.find_scene();
    if let Some(mat_comp) = c.find_jigsaw_mat(){
        // Check for jigsaw end on F3.
        for (player_id , input, player) in CompIter2::<InputComp, PlayerComp>::new(c) {
            if input.inputs.primitive.is_keycode_pressed(VirtualKeyCode::F3){
                scene.next = JigsawSceneType::Lobby;
            }
        }
    }
}



