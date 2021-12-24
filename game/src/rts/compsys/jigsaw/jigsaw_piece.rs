
use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use std::ops::Div;

use ggez::event::MouseButton;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct JigsawPieceComp{
    pub coords: PointInt,
    pub image: String,
}
impl JigsawPieceComp{
    pub fn get_correct_pos(&self) -> PointFloat{
        return PointFloat::new(self.coords.x as f32 * JIGSAW_PIECE_SIZE,
                               self.coords.y as f32 * JIGSAW_PIECE_SIZE);
    }
}

pub fn jigsaw_piece_sys<C>() -> System<C>{
    System{
        run,
        name: "jigsaw_piece"
    }
}
fn run<C>(c: &mut CompStorage<C>, ent_changes: &mut EntStructureChanges<C>, meta: &SimMetadata){

}

