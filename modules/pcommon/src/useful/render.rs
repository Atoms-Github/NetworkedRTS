use crate::*;
use ggez::{graphics, Context};
use ggez::graphics::{DrawParam, Text, Color, Mesh, MeshBuilder, Drawable, Rect};
use nalgebra::Point2;
use std::collections::BTreeMap;
use winit::event::VirtualKeyCode;
use std::fmt;
use becs::superb_ecs::SuperbEcs;
use crate::render_resources::GgEzRenderResources;
use crate::cool_batcher::CoolBatcher;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RenderComp{
    pub z: u16,
    pub only_render_owner: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SimpleViewerComp {
    pub texture: RenderTexture,
    pub shape: RenderShape,
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


pub fn simple_render(cool_batcher: &mut CoolBatcher, ecs: &mut SuperbEcs, player_entity_id: GlobalEntityID){
    let player_camera = ecs.c.get::<CameraComp>(player_entity_id).unwrap();
    let player_input = ecs.c.get::<InputComp>(player_entity_id).unwrap();

    // Draw entities.
    for (entity_id, position, render, viewer, size) in
    CompIter4::<PositionComp, RenderComp, SimpleViewerComp, SizeComp>::new(&ecs.c){
        let (on_screen_pos, on_screen_size) = player_camera.get_as_screen_transform(&ecs.c, entity_id);
        match &viewer.shape{
            RenderShape::Circle => {
                let radius = on_screen_size.x;
                match &viewer.texture{
                    RenderTexture::Color(r,g,b,a) => {
                        cool_batcher.add_circle(&on_screen_pos, radius, Color::new(*r,*g,*b,*a), render.z);
                    }
                    RenderTexture::Image(_) => {panic!("Render image circle isn't supported! (yet loh)")}
                }
            }
            RenderShape::Rectangle => {
                match &viewer.texture{
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
                }
            }
            RenderShape::Text(text) => {unimplemented!()}
        }
    }


    let mut test_param = DrawParam::new();
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