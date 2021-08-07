pub use crate::bibble::data::data_types::*;
use crate::rts::GameState;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::bibble::effect_resolver::revolver::Revolver;


impl<'a> Revolver<'a> {
    pub fn resolve_utu(&mut self, c: &mut CompStorage, effect: EffectUnitToUnit, source: GlobalEntityID, target: GlobalEntityID) {
        match effect {
            EffectUnitToUnit::INSTA_AFFECT_TARGET(to_unit) => {
                self.resolve_tu(to_unit, target);
            }
            _ => {
                unimplemented!()
            }
        }
    }
}