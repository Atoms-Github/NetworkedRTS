
use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use std::ops::Div;

pub const JIGSAW_PIECE_SIZE : f32 = 200.0;

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
    for (player_id, jigsaw_player, input, camera) in CompIter3::<JigsawPlayerComp, InputComp, CameraComp>::new(c){
        if let Some(held_piece) = jigsaw_player.held_item.clone(){

            if input.inputs.mouse_event == RtsMouseEvent::MouseUp{
                let piece_comp = c.get_unwrap::<JigsawPieceComp>(held_piece);
                let correct_place = piece_comp.coords.map(|i|{i as f32 * JIGSAW_PIECE_SIZE});
                let actual_place = c.get_mut_unwrap::<PositionComp>(held_piece);
                if actual_place.pos.dist(&correct_place) < JIGSAW_PIECE_SIZE / 2.0{
                    actual_place.pos = correct_place;
                }

                jigsaw_player.held_item = None;

            }
        }else{
            for (piece_id, jigsaw_piece, clickable) in CompIter2::<JigsawPieceComp, ClickableComp>::new(c){
                if Some(player_id) == clickable.clicking_on{
                    jigsaw_player.held_item  = Some(piece_id);
                }
            }
        }
        if let Some(held_piece) = jigsaw_player.held_item{
            c.get_mut_unwrap::<PositionComp>(held_piece).pos += input.inputs.mouse_moved.clone().div(camera.zoom);
        }

    }
}

