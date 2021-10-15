
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


pub static JIGSAW_SYS: System = System{
    run,
    name: "jigsaw_piece"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    
}

