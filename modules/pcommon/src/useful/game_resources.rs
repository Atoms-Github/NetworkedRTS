use std::collections::HashMap;
use image::{ImageBuffer, RgbImage, open, Rgba, RgbaImage};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::path::Path;
use std::fs;



lazy_static! {
    pub static ref GAME_RESOURCES: Mutex<GameResources> = {
        Mutex::new(GameResources{
            loaded: Default::default(),
        })
    };
}


pub struct GameResources{
    loaded: HashMap<String, RgbaImage>,
}
impl GameResources{
    pub fn get_image(&mut self, filename: String) -> &RgbaImage<>{
        let filename = format!("./resources/images/{}", filename);
        if !self.loaded.contains_key(&filename){
            let loaded = open(filename.clone()).expect("Error loading image.");
            let image = loaded.into_rgba8();
            self.loaded.insert(filename.clone(), image);
        }
        return self.loaded.get(&filename).unwrap();
    }
}