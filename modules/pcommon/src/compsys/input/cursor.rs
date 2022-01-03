use crate::*;
use std::ops::Div;

use ggez::event::MouseButton;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CursorComp{
    pub player: GlobalEntityID,
}

pub static CURSOR_SYS: System = System{
    run,
    name: "cursor"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    for (cursor_id, cursor_comp, position) in CompIter2::<CursorComp, PositionComp>::new(c){
        let (camera, input) = c.get2_unwrap::<CameraComp, InputComp>(cursor_comp.player);
        position.pos = input.mouse_pos_game_world.clone();
    }
}

