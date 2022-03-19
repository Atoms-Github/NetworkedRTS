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
            c.get_mut_unwrap::<OwnsCommanderComp>(player_id).selected_race = race_button.race;
        }
    }
}

