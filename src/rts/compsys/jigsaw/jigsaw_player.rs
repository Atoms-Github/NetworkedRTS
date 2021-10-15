
use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use std::ops::Div;

pub const JIGSAW_PIECE_SIZE : f32 = 50.0;

use ggez::event::MouseButton;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct JigsawPlayerComp{
    pub held_item: Option<GlobalEntityID>
}


pub static JIGSAW_SYS: System = System{
    run,
    name: "jigsaw_player"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){

}

