use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use ggez::graphics::Rect;
use std::ops::Div;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UIComp {
    // pub anchor: AnchorCorner,
    // pub offset: PointFloat,
    pub useless: bool,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum AnchorCorner{
    TOP_LEFT, TOP_RIGHT, BOTTOM_LEFT, BOTTOM_RIGHT,
}

pub static UI_SYS: System = System{
    run,
    name: "ui"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    // // Increment time since use timers.
    // for (ui_id, ui, position, owned_comp)
    // in CompIter2::<UIComp, PositionComp>::new(c) {
    //
    // }
}


