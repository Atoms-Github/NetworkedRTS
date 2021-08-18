pub use crate::bibble::data::data_types::*;
use crate::rts::GameState;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::LifeComp;
use crate::bibble::effect_resolver::revolver::Revolver;
use crate::pub_types::PointFloat;


impl<'a> Revolver<'a>{
    pub fn revolve_unit_to_point(&mut self, data: &GameData, effect: &EffectUnitToPoint, source: GlobalEntityID,
                                 target: PointFloat){
        match effect{
            EffectUnitToPoint::NOTHING => {}
            _ => {
                unimplemented!()
            }
        }
    }
}

