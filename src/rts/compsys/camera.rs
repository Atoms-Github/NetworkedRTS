use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;

pub struct CameraComp{
    pub translation: PointFloat,
    pub zoom: f32,
}

impl CameraComp{
    pub fn get_as_screen_coords<'a>(&self, ecs: &'a CompStorage, entity_id: GlobalEntityID) -> (&'a PointFloat, &'a PointFloat){

        let position : &'a PointFloat = &ecs.get::<PositionComp>(entity_id).unwrap().pos;
        let size : &'a PointFloat = &ecs.get::<SizeComp>(entity_id).unwrap().size;
// &PointFloat::new(100.0, 100.0)
        return (position, size);
    }
    pub fn game_space_to_screen_space(&self, coords: PointFloat) -> PointFloat{
        return coords - &self.translation;
    }
    pub fn game_size_to_screen_size(&self, coords: PointFloat) -> PointFloat{
        return coords * self.zoom;
    }
}