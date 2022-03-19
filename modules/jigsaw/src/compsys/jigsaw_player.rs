
use crate::*;
use std::ops::Div;

use ggez::event::MouseButton;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct JigsawPlayerComp{
    pub held_piece: Option<GlobalEntityID>
}


pub static JIGSAW_PLAYER_SYS: System = System{
    run,
    name: "jigsaw_player"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    let mut mat_comp = c.find_jigsaw_mat();
    for (player_id, player, jigsaw_player, input, camera)
    in CompIter4::<PlayerComp, JigsawPlayerComp, InputComp, CameraComp>::new(c){
        if !player.connected && mat_comp.is_none(){
            continue;
        }
        if let Some(held_piece) = jigsaw_player.held_piece.clone(){
            if input.inputs.mouse_event == NiceMouseEvent::MouseUp{

                let held_render_comp = c.get_mut_unwrap::<RenderComp>(held_piece);

                held_render_comp.z = mat_comp.as_ref().unwrap().next_piece_z;

                mat_comp.as_mut().unwrap().next_piece_z += 1;
                if mat_comp.as_mut().unwrap().next_piece_z >= JZValue::JigsawPieceHeld.g(){
                    mat_comp.as_mut().unwrap().next_piece_z = JZValue::GamePiece.g() + 1;
                }


                let piece_comp = c.get_unwrap::<JigsawPieceComp>(held_piece);
                let correct_place = piece_comp.get_correct_pos();
                let actual_place = c.get_mut_unwrap:: <PositionComp>(held_piece);
                // Try for teleport both pieces to correct place.
                for (piece_id, matched_to_piece, matched_pos, render) in CompIter3::<JigsawPieceComp, PositionComp, RenderComp>::new(c){
                    if piece_id != held_piece{
                        let real_dist = (matched_pos.clone().pos - &actual_place.pos) as PointFloat;
                        let coords_diff = (matched_to_piece.coords.clone() - &piece_comp.coords) as PointInt;
                        if coords_diff.x.abs() + coords_diff.y.abs() <= 1{
                            let actual_coords_place_diff = coords_diff.clone().map(|i| {i as f32 * JIGSAW_PIECE_SIZE}) as PointFloat;
                            if actual_coords_place_diff.dist(&real_dist) < JIGSAW_PIECE_SIZE / 3.0{
                                render.z = JZValue::BelowGamePiece.g();
                                held_render_comp.z = JZValue::BelowGamePiece.g();
                                // Teleport both to correct place.
                                matched_pos.pos = matched_to_piece.get_correct_pos();
                                actual_place.pos = correct_place.clone();
                            }
                        }
                    }
                }

                jigsaw_player.held_piece = None;

            }
        }else{
            for (piece_id, jigsaw_piece, clickable, pos, render) in
            CompIter4::<JigsawPieceComp, ClickableComp, PositionComp, RenderComp>::new(c){
                if Some(player_id) == clickable.clicking_on
                && jigsaw_piece.get_correct_pos() != pos.pos.clone(){
                    jigsaw_player.held_piece = Some(piece_id);
                    render.z = JZValue::JigsawPieceHeld.g();
                }
            }
        }
        if let Some(held_piece) = jigsaw_player.held_piece {
            // TODO0: Try just setting to world position.
            c.get_mut_unwrap::<PositionComp>(held_piece).pos += input.inputs.mouse_moved.clone().div(camera.zoom);
        }

    }
}

