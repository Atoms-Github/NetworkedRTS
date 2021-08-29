use crate::pub_types::{HashType, FrameIndex, PlayerID, RenderResourcesPtr, PointFloat};
use crate::netcode::{InfoForSim, PlayerInputs};
use ggez::{*};
use std::sync::Arc;
use crate::ecs::{ActiveEcs, GlobalEntityID};
use ggez::graphics::{DrawParam, Text};
use nalgebra::Point2;
use crate::ecs::pending_entity::PendingEntity;
use serde_closure::internal::std::future::Pending;
pub use crate::utils::gett;
use crate::ecs::superb_ecs::System;
use crate::rts::compsys::player::{PlayerComp};
use crate::rts::compsys::*;
use crate::bibble::data::data_types::{GameData, RaceID};
use crate::bibble::effect_resolver::revolver::Revolver;
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
        resources_dir.push("resources");

        let resources_dir_canon = std::fs::canonicalize(resources_dir.clone()).unwrap();
        let mut images_dir = resources_dir.clone();
        images_dir.push("images");
        let images_dir_canon = std::fs::canonicalize(images_dir.clone()).unwrap();
        for file in WalkDir::new(images_dir).into_iter().skip(1) /*Skip itself*/{
            let mid = file.unwrap();
            let full_path = std::fs::canonicalize(mid.path()).unwrap();
            let from_resources = full_path.strip_prefix(resources_dir_canon.clone()).unwrap();
            let from_images = full_path.strip_prefix(images_dir_canon.clone()).unwrap();
            let mut short = format!("\\{:?}", from_resources);
            short = short.replace("\"", "");
            let image = ggez::graphics::Image::new(ctx, short).unwrap();
            self.images.insert(from_images.to_str().unwrap().to_string(), image);
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