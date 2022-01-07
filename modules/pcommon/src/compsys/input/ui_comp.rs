use std::ops::Div;

use ggez::event::MouseButton;
use ggez::graphics::Rect;

use netcode::common::net_game_state::StaticFrameData;

use crate::*;

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
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    // // Increment time since use timers.
    // for (ui_id, ui, position, owned_comp)
    // in CompIter2::<UIComp, PositionComp>::new(c) {
    //
    // }
}


