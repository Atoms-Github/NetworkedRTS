use ggez::graphics::{Color, DrawParam, Image, DrawMode, Rect, MeshBuilder, Drawable, FilterMode, Text, draw};
use std::collections::HashMap;
use crate::pub_types::{PointFloat, RenderResourcesPtr};
use ggez::{Context, graphics};
use ggez::graphics::spritebatch::SpriteBatch;
use itertools::Itertools;
use mint::Point2;
use crate::pub_types::*;
use std::ops::Div;

struct BatcherImage{
    path: String,
    top_left: PointFloat,
    bottom_left: PointFloat,
}
#[derive(Default)]
pub struct CoolBatcher{
    layers: HashMap<u8, RenderLayer>,
}

pub struct MyDrawParams{
    pub pos: PointFloat,
    pub size: PointFloat,
}
// impl MyDrawParams{
//     pub fn to_their_draw_params(self) -> DrawParam{
//         let mut params = DrawParam::new();
//         return params;
//     }
// }
#[derive(Default)]
pub struct RenderLayer{
    images: HashMap<String, Vec<MyDrawParams>>,
    rectangles: Vec<(MyDrawParams, Color)>,
    circles: Vec<(PointFloat, f32, Color)>,
    texts: Vec<(PointFloat, String, Color)>,
}


impl CoolBatcher{
    pub fn new() -> Self{
        return Self::default()
    }
    pub fn add_rectangle_rect(&mut self, rect: MyDrawParams, color: Color, z: u8){
        self.layers
            .entry(z)
            .or_default().rectangles.push((rect, color));
    }
    pub fn add_rectangle(&mut self, position: &PointFloat, size: &PointFloat, color: Color, z: u8){
        self.add_rectangle_rect(MyDrawParams{
            pos: position.clone(),
            size: size.clone()
        }, color, z)
    }
    pub fn add_progress_bar(&mut self, centre: &PointFloat, height: f32, value: f32, max: f32, color_value: Color, color_max: Color, z: u8){
        self.add_rectangle_rect(MyDrawParams{
            pos: centre.clone(),
            size: PointFloat::new(value, height)
        }, color_value, z + 1);
        self.add_rectangle_rect(MyDrawParams{
            pos: centre.clone(),
            size: PointFloat::new(max, height)
        }, color_max, z);

    }
    pub fn add_circle(&mut self, position: &PointFloat, size: f32, color: Color, z: u8){
        self.layers
            .entry(z)
            .or_default().circles.push((position.clone(), size, color));
    }
    pub fn add_image(&mut self, filename: String, draw_param: MyDrawParams, z: u8){
        self.layers
            .entry(z)
            .or_default().images
            .entry(filename)
            .or_default()
            .push(draw_param);
    }
    pub fn add_text(&mut self, position: PointFloat, text: String, color: Color, z: u8){
        self.layers
            .entry(z)
            .or_default().texts.push((position, text, color));
    }
    pub fn gogo_draw(self, ctx: &mut Context, res: &RenderResourcesPtr){
        // Iterate spritebatches ordered by z and actually render each of them
        for (z, render_layer) in self.layers
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        {
            // Draw images:
            for (image_name, draw_params) in render_layer.images {
                let image = res.images.get(&image_name).expect(format!("Can't find image {}", image_name).as_str()).clone();
                let image_dimensions = PointFloat::new(image.dimensions().w, image.dimensions().h);
                let mut sprite_batch = SpriteBatch::new(image);

                for my_draw_params in draw_params {
                    // Work out image scale from size.
                    let one_pixel_scale : PointFloat = PointFloat::new(1.0,1.0).component_div(&image_dimensions);

                    let mut draw_params = DrawParam::new();
                    let top_left_corner = (my_draw_params.pos - (my_draw_params.size.clone().div(2.0))).to_point();
                    draw_params = draw_params.dest(top_left_corner);
                    draw_params = draw_params.scale(one_pixel_scale.component_mul(&my_draw_params.size));
                    sprite_batch.add(draw_params);
                }

                graphics::draw(ctx, &sprite_batch, graphics::DrawParam::new()).unwrap()
            }
            // Draw rectangles:
            if render_layer.rectangles.len() > 0{
                let mut builder = MeshBuilder::new();
                for (my_draw_params, color) in render_layer.rectangles.into_iter() {
                    let top_left_corner = (my_draw_params.pos - (my_draw_params.size.clone().div(2.0))).to_point();
                    let rect = ggez::graphics::Rect::new(top_left_corner.x, top_left_corner.y, my_draw_params.size.x, my_draw_params.size.y);
                    builder.rectangle(graphics::DrawMode::fill(), rect, color).unwrap();
                }
                builder.build(ctx).unwrap().draw(ctx, DrawParam::new()).unwrap();
            }


            // Draw text:
            for (position, text_str, color) in render_layer.texts.into_iter() {
                let text_display = Text::new(text_str);

                graphics::draw(
                    ctx,
                    &text_display,
                    (nalgebra::Point2::from(position), color),
                ).unwrap();
                // TODO3: Batch seems to be slow for some reason.
                // let text_text = ggez::graphics::Text::new(text_str);
                // let my_point = nalgebra::Point2::from(position);
                // ggez::graphics::queue_text(ctx, &text_text, my_point, Some(color));
            }
            // ggez::graphics::draw_queued_text(ctx, DrawParam::new(), None, FilterMode::Linear).unwrap();
        }
    }
}