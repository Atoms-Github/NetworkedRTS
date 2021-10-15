use std::collections::{BTreeSet};
use crate::ecs::GlobalEntityID;
use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use ggez::graphics::Rect;
use std::ops::Div;
use serde::*;
use crate::bibble::data::data_types::RaceID;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MapButtonComp {
    pub selected: bool,
    pub map: String,
}

pub static MAP_BUTTON_SYS: System = System{
    run,
    name: "map_button_sys"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    for (button_id, button, map_button) in CompIter2::<ClickableComp, MapButtonComp>::new(c){
        if let Some(player_id) = button.clicking_on{
            for (map_button_id, map_button) in CompIter1::<MapButtonComp>::new(c){
                map_button.selected = false;
            }
            map_button.selected = true;
        }
    }
}

