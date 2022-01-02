use std::collections::{BTreeSet};
use crate::ecs::GlobalEntityID;
use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use game::pub_types::{PointFloat, PlayerID};
use bibble::::*;
use ggez::graphics::Rect;
use std::ops::Div;
use serde::*;
use game::bibble::data::data_types::noneffects::RaceID;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RaceButtonComp {
    pub race: RaceID,
}

pub static RACE_BUTTON_SYS: System = System{
    run,
    name: "race_button_sys"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    for (button_id, button, race_button) in CompIter2::<ClickableComp, RaceButtonComp>::new(c){
        if let Some(player_id) = button.clicking_on{
            c.get_mut_unwrap::<PlayerComp>(player_id).race = race_button.race;
            // // Clear player's selection from all other buttons.
            // for (other_button_id, race_button) in CompIter1::<RaceButtonComp>::new(c){
            //     // Avoid any unsafe issues.
            //     if other_button_id != button_id{
            //         race_button.clicked_on.remove(&player_id);
            //     }
            // }
            // race_button.clicked_on.insert(player_id);
        }
    }
}

