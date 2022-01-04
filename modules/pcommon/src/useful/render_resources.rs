use netcode::*;
use ggez::{*};
use std::sync::Arc;
use ggez::graphics::{DrawParam, Text};
use nalgebra::Point2;
pub use crate::utils::gett;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use walkdir::WalkDir;


// No clone or serde.
pub struct RenderResources {
    pub images: HashMap<String, graphics::Image>
}
impl RenderResources{
    fn load_me(&mut self, ctx: &mut Context){
        // let path = "C:\\_C_\\Home\\Produce\\Code\\Projects\\Rust\\BigYoshis\\LiteralPoggySource\\target\\release\\resources\\images\\factory.jpg";
        // let image = ggez::graphics::Image::new(ctx, path).unwrap();
        let mut resources_dir = std::env::current_exe().unwrap();
        resources_dir.pop();
        resources_dir.push("../../../../resources");

        let resources_dir_canon = std::fs::canonicalize(resources_dir.clone()).unwrap();
        let mut images_dir = resources_dir.clone();
        images_dir.push("images");
        let images_dir_canon = std::fs::canonicalize(images_dir.clone()).unwrap();
        for file in WalkDir::new(images_dir).into_iter(){
            let mid = file.unwrap();
            let full_path = std::fs::canonicalize(mid.path()).unwrap();
            if full_path.is_file(){
                let from_resources = full_path.strip_prefix(resources_dir_canon.clone()).unwrap();
                let from_images = full_path.strip_prefix(images_dir_canon.clone()).unwrap();
                let mut short = format!("\\{:?}", from_resources);
                short = short.replace("\"", "");
                let image = ggez::graphics::Image::new(ctx, short).unwrap();
                self.images.insert(from_images.file_name().unwrap().to_str().unwrap().to_string(), image);
            }


        }
    }
    pub fn load(ctx: &mut Context) -> Self{
        let mut my_self = Self{
            images: Default::default()
        };
        my_self.load_me(ctx);
        return my_self;

    }
}