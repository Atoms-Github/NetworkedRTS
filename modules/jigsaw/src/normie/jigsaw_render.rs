use crate::*;
use ggez::{graphics, Context};
use ggez::graphics::{DrawParam, Text, Color, Mesh, MeshBuilder, Drawable, Rect};
use nalgebra::Point2;
use std::collections::BTreeMap;
use std::fmt;
use becs::superb_ecs::SuperbEcs;

pub fn jigsaw_render(ecs: &mut SuperbEcs, player_entity_id: GlobalEntityID){
    let mut cool_batcher = CoolBatcher::new();

    let player_camera = ecs.c.get::<CameraComp>(player_entity_id).unwrap();
    let player_input = ecs.c.get::<InputComp>(player_entity_id).unwrap();

    // Draw players.
    let mut y = 100.0;
    for (entity_id, player) in CompIter1::<PlayerComp>::new(&ecs.c){
        if player.connected{
            y += 50.0;
            cool_batcher.add_text(PointFloat::new(20.0, y),
                                  player.name.clone(), Color::from_rgb(0,0,0), JZValue::UI.g());
        }
    }


    for (entity_id, _, render, _, piece) in
    CompIter4::<PositionComp, RenderComp, SizeComp, JigsawPieceComp>::new(&ecs.c){
        let (on_screen_pos, on_screen_size) = player_camera.get_as_screen_transform(&ecs.c, entity_id);

        let mut their_params = DrawParam::new();
        cool_batcher.add_image_part(piece.image.clone(), MyDrawParams{
            pos: on_screen_pos.clone(),
            size: on_screen_size.clone(),
        }, Rect::new(piece.coords.x as f32 * JIGSAW_PIECE_SIZE,
                     piece.coords.y as f32 * JIGSAW_PIECE_SIZE, JIGSAW_PIECE_SIZE, JIGSAW_PIECE_SIZE), render.z);
    }

}

