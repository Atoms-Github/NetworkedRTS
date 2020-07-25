use std::sync::{Arc, RwLock};

pub type PlayerID = u32;
pub type FrameIndex = usize;
pub type PointFloat = nalgebra::Point2<f32>;
pub type ThreadCloser = ();
pub type ArcRw<T> = Arc<RwLock<T>>;