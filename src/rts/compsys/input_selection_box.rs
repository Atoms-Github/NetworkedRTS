

use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::rts::game::game_state::{GameResources, ARENA_ENT_ID};
use ggez::event::MouseButton;

pub static INPUT_SELECTION_BOX: System<ResourcesPtr> = System{
    run
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    //: (usize, &mut CameraComp, &mut InputComp)
    for group in CompIter1::<CameraComp>::new(c) {
        let (id, camera) : (usize, &mut CameraComp) = group;

    }
    for (player_id ,camera, input) in CompIter2::<CameraComp, InputComp>::new(c) {
        match input.mode.clone() {
            InputMode::None => {
                if input.hovered_entity.is_none() {
                    if input.inputs.mouse_event == RtsMouseEvent::MouseDown(MouseButton::Left) {

                        ent_changes.new_entities.push(PendingEntity::new_sel_box(player_id, input.mouse_pos_game_world.clone()));
                        input.mode = InputMode::SelectionBox;
                    }
                }
            }
            InputMode::SelectionBox => {
                if input.inputs.mouse_event == RtsMouseEvent::MouseUp {
                    input.mode = InputMode::None;
                    for (box_id, sel, owner) in CompIter2::<SelBoxComp, OwnedComp>::new(c) {
                        if owner.owner == player_id{
                            ent_changes.deleted_entities.push(box_id);
                        }
                    }

                }
            }
            _ => {}
        }
    }
}

