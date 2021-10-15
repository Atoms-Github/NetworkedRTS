
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
impl JigsawPieceComp{
    pub fn get_nearby_connected(&self, c: CompStorage) -> Vec<GlobalEntityID>{
        vec![]
    }
}

pub static JIGSAW_PIECE_SYS: System = System{
    run,
    name: "jigsaw_piece"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){

}

