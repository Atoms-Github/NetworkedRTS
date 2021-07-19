use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use crate::rts::game::game_state::{ARENA_ENT_ID, GameResources};
use ggez::graphics::Rect;


pub struct SelBoxComp{
}
pub static SELECTION_BOX: System<ResourcesPtr> = System{
    run
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (sel_box_id, sel_box, position, size, owned) in CompIter4::<SelBoxComp, PositionComp, SizeComp, OwnedComp>::new(c) {
        let mouse_pos = c.get::<InputComp>(owned.owner).unwrap().mouse_pos_game_world.clone();
        size.size = mouse_pos - position.pos;
    }

    for (player_id , input) in CompIter1::<InputComp>::new(c) {
        match input.mode.clone() {
            // Spawning it.
            InputMode::None | InputMode::UnitsSelected => {
                check_create_box(c, ent_changes, player_id, input)
            }
            // Deleting it.
            InputMode::SelectionBox => {
                check_delete_box(c, player_id)
            }
            _ => {}
        }
    }
}

fn check_delete_box(c: &CompStorage, player_id: GlobalEntityID) {
    let input = c.get1_unwrap::<InputComp>(player_id);
    if input.inputs.mouse_event == RtsMouseEvent::MouseUp {
        if let Some(box_id) = get_box(c, player_id){
            // ent_changes.deleted_entities.push(box_id);
            let any_selected = select_units_in_box(c, box_id);
            if any_selected{
                input.mode = InputMode::UnitsSelected;
            }
        }
    }
}

fn check_create_box(c: &CompStorage, ent_changes: &mut EntStructureChanges, player_id: GlobalEntityID, input: &mut InputComp) {
    let input = c.get1_unwrap::<InputComp>(player_id);
    if input.hovered_entity.is_none() {
        if input.inputs.mouse_event == RtsMouseEvent::MouseDown(MouseButton::Left) {
            ent_changes.new_entities.push(PendingEntity::new_sel_box(player_id, input.mouse_pos_game_world.clone()));
            input.mode = InputMode::SelectionBox;

            deselect_all(c, player_id)
        }
    }
}
fn get_box(c: &CompStorage, player_id: GlobalEntityID) -> Option<GlobalEntityID>{
    for (box_id, sel, size, position, owner) in CompIter4::<SelBoxComp, SizeComp, PositionComp, OwnedComp>::new(c) {
        if owner.owner == player_id {
            return Some(box_id);
        }
    }
    return None;
}
fn deselect_all(c: &CompStorage, player_id: GlobalEntityID) {
    for (box_id, sel, owner) in CompIter2::<SelectableComp, OwnedComp>::new(c) {
        if owner.owner == player_id {
            sel.is_selected = false;
        }
    }
}

fn select_units_in_box(c: &CompStorage, box_id: GlobalEntityID) -> bool{
    let (owned_box, position_box, size_box) = c.get3_unwrap::<OwnedComp, PositionComp, SizeComp>(box_id);
    let sel_box_rect = Rect{
        x: position_box.pos.x,
        y: position_box.pos.y,
        w: size_box.size.x,
        h: size_box.size.y,
    };

    let mut selected_any = false;
    for (unit_id, sel_unit, position_unit, owned_unit) in CompIter3::<SelectableComp, PositionComp, OwnedComp>::new(c) {
        let as_vec = vec![position_unit.pos.x, position_unit.pos.y];
        let mint_unit_pos = ggez::mint::Point2::from_slice(&as_vec[..]);
        if owned_unit.owner == owned_box.owner && sel_box_rect.contains(mint_unit_pos){
            sel_unit.is_selected = true;
            selected_any = true;
        }
    }
    return selected_any;
}