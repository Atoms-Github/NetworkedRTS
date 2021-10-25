
use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use std::ops::Div;

pub const JIGSAW_PIECE_SIZE : f32 = 75.0;

use ggez::event::MouseButton;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct JigsawPlayerComp{
    pub held_item: Option<GlobalEntityID>
}


pub static JIGSAW_PLAYER_SYS: System = System{
    run,
    name: "jigsaw_player"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    for (player_id, player, jigsaw_player, input, camera) in CompIter4::<PlayerComp, JigsawPlayerComp, InputComp, CameraComp>::new(c){
        if !player.connected{
            continue;
        }
        if let Some(held_piece) = jigsaw_player.held_item.clone(){
            if input.inputs.mouse_event == RtsMouseEvent::MouseUp{
                let held_render_comp = c.get_mut_unwrap::<RenderComp>(held_piece);
                held_render_comp.z = 100;
                let piece_comp = c.get_unwrap::<JigsawPieceComp>(held_piece);
                let correct_place = piece_comp.get_correct_pos();
                let actual_place = c.get_mut_unwrap:: <PositionComp>(held_piece);
                if actual_place.pos.dist(&correct_place) < JIGSAW_PIECE_SIZE / 2.0{
                    actual_place.pos = correct_place;
                    held_render_comp.z = 99;
                }else{
                    // Try for teleport both pieces to correct place.
                    for (piece_id, matched_to_piece, matched_pos, render) in CompIter3::<JigsawPieceComp, PositionComp, RenderComp>::new(c){
                        if piece_id != held_piece{
                            let real_dist = (matched_pos.clone().pos - &actual_place.pos) as PointFloat;
                            let coords_diff = (matched_to_piece.coords.clone() - &piece_comp.coords) as PointInt;
                            let actual_coords_place_diff = coords_diff.clone().map(|i| {i as f32 * JIGSAW_PIECE_SIZE}) as PointFloat;
                            if actual_coords_place_diff.dist(&real_dist) < JIGSAW_PIECE_SIZE / 10.0{
                                render.z = 99;
                                // Teleport both to correct place.
                                matched_pos.pos = matched_to_piece.get_correct_pos();
                                actual_place.pos = correct_place.clone();
                            }
                        }
                    }
                }



                jigsaw_player.held_item = None;

            }
        }else{
            for (piece_id, jigsaw_piece, clickable, pos, render) in
            CompIter4::<JigsawPieceComp, ClickableComp, PositionComp, RenderComp>::new(c){
                if Some(player_id) == clickable.clicking_on
                && jigsaw_piece.get_correct_pos() != pos.pos.clone(){
                    jigsaw_player.held_item  = Some(piece_id);
                    render.z = 101;
                }
            }
        }
        if let Some(held_piece) = jigsaw_player.held_item{
            c.get_mut_unwrap::<PositionComp>(held_piece).pos += input.inputs.mouse_moved.clone().div(camera.zoom);
        }

    }
}

