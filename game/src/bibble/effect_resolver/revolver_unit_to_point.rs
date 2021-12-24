pub use crate::bibble::data::data_types::*;
use crate::rts::GameStateJigsaw;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::{LifeComp, OwnedComp};
use crate::bibble::effect_resolver::revolver::Revolver;
use crate::pub_types::PointFloat;


impl<'a, C> Revolver<'a, C>{
    pub fn revolve_unit_to_point(&mut self, data: &GameData, effect: &EffectUnitToPoint, source: GlobalEntityID,
                                 target: &PointFloat){
        let owner = self.c.get1_unwrap::<OwnedComp>(source).owner;
        match effect{
            EffectUnitToPoint::NOTHING => {}
            EffectUnitToPoint::TO_POINT(to_point) => {
                self.revolve_to_point(data, to_point, target, owner);
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

