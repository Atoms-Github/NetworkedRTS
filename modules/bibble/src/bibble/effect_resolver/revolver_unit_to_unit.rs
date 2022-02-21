pub use crate::bibble::data::data_types::*;
use crate::rts::GameStateJigsaw;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::bibble::effect_resolver::revolver::Revolver;


impl<'a> Revolver<'a> {
    pub fn revolve_unit_to_unit(&mut self, data: &GameData, effect: &EffectUnitToUnit, source: GlobalEntityID, target: GlobalEntityID) {
        match effect {
            EffectUnitToUnit::INSTA_AFFECT_TARGET(to_unit) => {
                self.revolve_to_unit(data, to_unit, target);
            }
            EffectUnitToUnit::LAUNCH_SEEKING_PROJECTILE(proj_mould) =>{
                self.spawn_seeker_proj(data, proj_mould, source, target);
            }
        }
    }
}