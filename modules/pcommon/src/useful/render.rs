use crate::*;
use ggez::{graphics, Context};
use ggez::graphics::{DrawParam, Text, Color, Mesh, MeshBuilder, Drawable, Rect};
use crate::utils::gett;
use nalgebra::Point2;
use std::collections::BTreeMap;
use winit::event::VirtualKeyCode;
use std::fmt;
use becs::superb_ecs::SuperbEcs;
use crate::render_resources::RenderResources;
use crate::cool_batcher::CoolBatcher;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RenderComp{
    pub z: u16,
    pub texture: RenderTexture,
    pub shape: RenderShape,
    pub only_render_owner: bool,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum RenderTexture{
    Color(f32, f32, f32, f32),
    Image(String),
    // Jigsaw(String, PointInt),
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum RenderShape{
    Circle,
    Rectangle,
    Text(String)
}


pub fn simples_render(ecs: &mut SuperbEcs, ctx: &mut Context, res: &RenderResources, player_entity_id: GlobalEntityID){
    let mut cool_batcher = CoolBatcher::new();

    let player_camera = ecs.c.get::<CameraComp>(player_entity_id).unwrap();
    let player_input = ecs.c.get::<InputComp>(player_entity_id).unwrap();

    // Draw entities.
    for (entity_id, position, render, size) in
    CompIter3::<PositionComp, RenderComp, SizeComp>::new(&ecs.c){
        let (on_screen_pos, on_screen_size) = player_camera.get_as_screen_transform(&ecs.c, entity_id);
        match &render.shape{
            RenderShape::Circle => {
                let radius = ecs.c.get_unwrap::<SizeComp>(entity_id).size.x;
                match &render.texture{
                    RenderTexture::Color(r,g,b,a) => {
                        cool_batcher.add_circle(&on_screen_pos, radius, Color::new(*r,*g,*b,*a), render.z);
                    }
                    RenderTexture::Image(_) => {panic!("Render image circle isn't supported! (yet loh)")}
                    // RenderTexture::Jigsaw(_, _) => {panic!("Render jigsaw circle isn't supported! (yet loh)")}
                }
            }
            RenderShape::Rectangle => {
                match &render.texture{
                    RenderTexture::Color(r,g,b,a) => {
                        cool_batcher.add_rectangle_rect(MyDrawParams{
                            pos: on_screen_pos.clone(),
                            size: on_screen_size.clone()
                        },
                                                        Color::new(*r,*g,*b,*a), render.z);
                    }
                    RenderTexture::Image(image_name) => {
                        let my_draw_params = MyDrawParams{
                            pos: on_screen_pos.clone(),
                            size: on_screen_size.clone(),
                        };
                        cool_batcher.add_image(image_name.clone(), my_draw_params, render.z);
                    }
                    // RenderTexture::Jigsaw(landscape_name, piece_coords) => {
                    //     let mut their_params = DrawParam::new();
                    //     cool_batcher.add_image_part(landscape_name.clone(), MyDrawParams{
                    //         pos: on_screen_pos.clone(),
                    //         size: on_screen_size.clone(),
                    //     }, Rect::new(piece_coords.x as f32 * JIGSAW_PIECE_SIZE,
                    //                  piece_coords.y as f32 * JIGSAW_PIECE_SIZE, JIGSAW_PIECE_SIZE, JIGSAW_PIECE_SIZE), render.z);
                    // }
                }
            }
            RenderShape::Text(text) => {unimplemented!()}
        }
    }


    let mut test_param = DrawParam::new();
    // cool_batcher.add_image("factory.jpg".to_string(), test_param, 5);
    cool_batcher.gogo_draw(ctx, res);


}


fn draw_text(ctx: &mut Context, on_screen_pos: PointFloat, text: String, color: graphics::Color) {
    let text_display = Text::new(text);

    graphics::draw(
        ctx,
        &text_display,
        (Point2::new(on_screen_pos.x, on_screen_pos.y), color),
    ).unwrap();
}

fn draw_rect(ctx: &mut Context, color: Color, mesh: graphics::Rect){
    let mode = graphics::DrawMode::fill();
    let mesh: graphics::Mesh = graphics::Mesh::new_rectangle(
        ctx,
        mode,
        mesh,
        color,
    ).unwrap();
    graphics::draw(
        ctx,
        &mesh,
        DrawParam::new(),
    ).unwrap();
}



trait MyToString{
    fn my_to_string(&self) -> String;
}
impl MyToString for VirtualKeyCode{
    fn my_to_string(&self) -> String {
        let test = format!("{:?}", self);
        return test;
    }
}