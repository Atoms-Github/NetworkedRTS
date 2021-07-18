use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::PointFloat;
use crate::rts::compsys::*;
use crate::rts::game::game_state::{ARENA_ENT_ID, GameResources};

pub struct SelBoxComp{
}
pub static INPUT_SELECTION_BOX: System<ResourcesPtr> = System{
    run
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){

    for (sel_box_id, sel_box, position, size, owned) in CompIter4::<SelBoxComp, PositionComp, SizeComp, OwnedComp>::new(c) {
        let mouse_pos = c.get::<InputComp>(owned.owner).unwrap().mouse_pos_game_world.clone();
        size.size = mouse_pos - position.pos;
    }


    // Spawning it.
    for (player_id ,camera, input) in CompIter2::<CameraComp, InputComp>::new(c) {
        match input.mode.clone() {
            InputMode::None | InputMode::UnitsSelected => {
                if input.hovered_entity.is_none() {
                    if input.inputs.mouse_event == RtsMouseEvent::MouseDown(MouseButton::Left) {

                        ent_changes.new_entities.push(PendingEntity::new_sel_box(player_id, input.mouse_pos_game_world.clone()));
                        input.mode = InputMode::SelectionBox;
                    }
                }
            }
            // Deleting it.
            InputMode::SelectionBox => {
                if input.inputs.mouse_event == RtsMouseEvent::MouseUp {
                    for (box_id, sel, owner) in CompIter2::<SelBoxComp, OwnedComp>::new(c) {
                        if owner.owner == player_id{
                            // ent_changes.deleted_entities.push(box_id);
                        }
                    }
                    for (box_id, sel, owner) in CompIter2::<SelectableComp, OwnedComp>::new(c) {
                        if owner.owner == player_id{
                            sel.is_selected = true;
                        }
                    }
                    input.mode = InputMode::UnitsSelected;

                }
            }
            _ => {}
        }
    }
}
