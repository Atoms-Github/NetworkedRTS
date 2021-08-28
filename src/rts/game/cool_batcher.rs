use ggez::graphics::{Color, DrawParam, Image};
use std::collections::HashMap;
use crate::pub_types::PointFloat;
use ggez::{Context, graphics};
use ggez::graphics::spritebatch::SpriteBatch;
use itertools::Itertools;

struct BatcherImage{
    path: String,
    top_left: PointFloat,
    bottom_left: PointFloat,
}
#[derive(Default)]
pub struct CoolBatcher{
    layers: HashMap<u8, RenderLayer>,
}
#[derive(Default)]
pub struct RenderLayer{
    images: HashMap<String, Vec<DrawParam>>,
}


impl CoolBatcher{
    pub fn new() -> Self{
        return Self::default()
    }
    pub fn add_rectangle(&mut self, rect: ggez::graphics::Rect, color: Color, z: u8){

    }
    pub fn add_image(&mut self, path: String, draw_param: DrawParam, z: u8){

        self.layers
            .entry(z)
            .or_default().images
            .entry(path)
            .or_default()
            .push(draw_param);
    }
    pub fn add_text(&mut self, text: String){

    }
    pub fn gogo_draw(self, ctx: &mut Context){
        // Iterate spritebatches ordered by z and actually render each of them
        for (z, render_layer) in self.layers
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        {
            for (image_path, draw_params) in &render_layer.images {
                let canonical = std::fs::canonicalize(format!("./resources/images/{}", image_path)).unwrap();
                let full_path = canonical.as_path();
                let test_full_path = "/images/factory.jpg";
                let image = Image::new(ctx, test_full_path).expect("expected image");
                let mut sprite_batch = SpriteBatch::new(image);

                for draw_param in draw_params.iter() {
                    sprite_batch.add(*draw_param);
                }

                graphics::draw(ctx, &sprite_batch, graphics::DrawParam::new())
                    .expect("expected render");
            }
        }
    }
}