use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use game::pub_types::{PointFloat, PlayerID};
use bibble::::*;
use ggez::graphics::Rect;
use std::ops::Div;

use game::bibble::data::data_types::{WeaponID, AbilityID};
use game::bibble::effect_resolver::revolver::Revolver;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UnitMetadataComp {

}


