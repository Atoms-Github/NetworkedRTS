use crate::*;

use ggez::event::{MouseButton, KeyCode};

use ggez::graphics::Rect;
use std::ops::Div;
use std::future::Pending;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SelBoxComp{
    pub starting_pos: PointFloat
}
pub static SELECTION_BOX_SYS: System = System{
    run,
    name: "selection_box"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    let scene = c.find_scene();
    for (sel_box_id, sel_box, position, size, owned) in CompIter4::<SelBoxComp, PositionComp, SizeComp, OwnedComp>::new(c) {
        let mouse_pos = c.get::<InputComp>(owned.owner).unwrap().mouse_pos_game_world.clone();
        let box_size_vec = mouse_pos - &sel_box.starting_pos;
        size.set_abs(&box_size_vec);
        position.pos = sel_box.starting_pos.clone() + box_size_vec.div(2.0);
    }

    let mut revolver = Revolver::new(c);

    for (player_id , input) in CompIter1::<InputComp>::new(c) {
        let input = c.get1_unwrap::<InputComp>(player_id);

        let data = player_id.get_player_tech_tree(c);
        match input.mode.clone() {
            // Spawning it.
            InputMode::None => {
                check_create_box(c, player_id, input);
            }
            // Deleting it.
            InputMode::SelectionBox => {
                check_delete_box(c, player_id);
            }
            _ => {}
        }
    }
    revolver.end().move_into(ent_changes);
}

fn check_delete_box(c: &mut CompStorage,  player_id: GlobalEntityID) {
    let input = c.get1_unwrap::<InputComp>(player_id);
    if input.inputs.mouse_event == RtsMouseEvent::MouseUp {
        if let Some(box_id) = get_box(c, player_id){
            c.req_delete_entity(box_id);
            let any_selected = select_units_in_box(c, box_id);
        }
        input.mode = InputMode::None;
    }
}

fn check_create_box(c: &mut CompStorage, player_id: GlobalEntityID, input: &mut InputComp) {
    let input = c.get1_unwrap::<InputComp>(player_id);
    // if input.hovered_entity.is_none() {
        if input.inputs.mouse_event == RtsMouseEvent::MouseDown(MouseButton::Left) {
            c.req_create_entity(PendingEntity::new_sel_box(player_id, input.mouse_pos_game_world.clone()));
            input.mode = InputMode::SelectionBox;

            deselect_all(c, player_id)
        }
    // }
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

    let sel_box_rect = size_box.get_as_rect(position_box);

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