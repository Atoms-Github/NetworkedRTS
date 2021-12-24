
use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use std::ops::Div;

use ggez::event::MouseButton;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CursorComp{
    pub player: GlobalEntityID,
}

pub fn cursor_sys<C>() -> System<C>{
    System{
        run,
        name: "cursor"
    }
}
fn run<C>(c: &mut CompStorage<C>, ent_changes: &mut EntStructureChanges<C>, meta: &SimMetadata){
    for (cursor_id, cursor_comp, position) in CompIter2::<CursorComp, PositionComp>::new(c){
        let (camera, input) = c.get2_unwrap::<CameraComp, InputComp>(cursor_comp.player);
        position.pos = input.mouse_pos_game_world.clone();
    }
}

