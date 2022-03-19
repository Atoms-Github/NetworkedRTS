use crate::*;
use std::ops::Div;

use ggez::event::MouseButton;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UninteractableComp {
    pub useless: bool,
}
