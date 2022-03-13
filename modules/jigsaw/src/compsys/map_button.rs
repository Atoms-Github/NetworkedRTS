use crate::*;
use std::collections::{BTreeSet};
use ggez::event::MouseButton;

use ggez::graphics::Rect;
use std::ops::Div;
use serde::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MapButtonComp {
    pub selected: bool,
    pub map: String,
}

pub static MAP_BUTTON_SYS: System = System{
    run,
    name: "map_button_sys"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    for (button_id, button, map_button, render) in CompIter3::<ClickableComp, MapButtonComp, RenderComp>::new(c){
        if let Some(player_id) = button.clicking_on{
            for (map_button_id, map_button) in CompIter1::<MapButtonComp>::new(c){
                map_button.selected = false;
            }
            map_button.selected = true;
        }
    }
}

