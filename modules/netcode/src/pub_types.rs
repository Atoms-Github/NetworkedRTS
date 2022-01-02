use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::common::input_state::InputState;
use ggez::graphics::Color;
use std::sync::{Arc, RwLock};
use nalgebra::U2;

pub type PlayerID = u32;
pub type HashType = u64;
pub type FrameIndex = usize;
pub type ArcRw<T> = Arc<RwLock<T>>;
pub type ServerEvents = Vec<ServerEvent>;
pub type PointFloat = nalgebra::VectorN<f32, U2>;
pub use crate::common::confirmed_data::ServerEvent;


pub struct SimMetadata{
    pub delta: f32,
    pub quality: SimQuality,
    pub frame_index: FrameIndex,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Copy)]
pub struct Shade(pub f32, pub f32, pub f32);

#[derive(PartialEq)]
pub enum SimQuality{
    DETERMA,
    HEAD
}
impl Shade{
    pub fn to_color(&self) -> Color{
        Color{
            r: self.0,
            g: self.1,
            b: self.2,
            a: 1.0
        }
    }
}
pub type PlayerInputs = HashMap<PlayerID, InputState>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InfoForSim {
    pub inputs_map: PlayerInputs,
    pub server_events: ServerEvents
}