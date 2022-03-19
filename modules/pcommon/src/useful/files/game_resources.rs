use std::collections::HashMap;
use image::{ImageBuffer, RgbImage, open, Rgba, RgbaImage};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::path::Path;
use std::fs;
use crate::FileCache;


#[derive(Default)]
pub struct LogicResources {
    images: FileCache<RgbaImage>,
}
impl LogicResources {
    pub fn get_image(&mut self, filename: String) -> &RgbaImage{
        let mut full = "images/".to_owned() + filename.as_str();
        return self.images.get(&full, |bytes|{
            let loaded = image::load_from_memory(bytes).expect("Can't parse image.");
            loaded.into_rgba8()
        });
    }
}