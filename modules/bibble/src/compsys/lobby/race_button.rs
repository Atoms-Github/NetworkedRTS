use crate::*;
use std::collections::{BTreeSet};
use ggez::event::MouseButton;

use ggez::graphics::Rect;
use std::ops::Div;
use serde::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RaceButtonComp {
    pub race: RaceID,
}

pub static RACE_BUTTON_SYS: System = System{
    run,
    name: "race_button_sys"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
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

