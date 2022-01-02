
use bibble::::jigsaw::jigsaw_game_state::*;
use bibble::::*;
use game::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::event::MouseButton;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SelectableComp {
    pub is_selected: bool
}