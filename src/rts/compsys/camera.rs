use crate::pub_types::PointFloat;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;

pub struct CameraComp{
    pub translation: PointFloat,
    pub zoom: f32,
}

impl CameraComp{
    pub fn get_as_screen_coords(&self, ecs: &CompStorage, entity_id: GlobalEntityID) -> (PointFloat, PointFloat){

        let position  = &ecs.get::<PositionComp>(entity_id).unwrap().pos.clone();
        let size  = &ecs.get::<SizeComp>(entity_id).unwrap().size.clone();

        let pos_screen = self.game_space_to_screen_space(*position);
        let size_screen = self.game_size_to_screen_size(*size);
        return (pos_screen, size_screen);
    }
    pub fn game_space_to_screen_space(&self, coords: PointFloat) -> PointFloat{
        return coords - &self.translation;
    }
    pub fn game_size_to_screen_size(&self, coords: PointFloat) -> PointFloat{
        return coords * self.zoom;
    }
}