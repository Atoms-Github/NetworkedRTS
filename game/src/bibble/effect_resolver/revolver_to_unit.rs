pub use crate::bibble::data::data_types::*;
use crate::rts::GameStateJigsaw;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::bibble::effect_resolver::revolver::Revolver;


impl<'a> Revolver<'a>{
    pub fn revolve_to_unit(&mut self, data: &GameData, effect: &EffectToUnit, target: GlobalEntityID){
        match effect{
            EffectToUnit::DAMAGE(damage) => {
                let life = self.c.get1_unwrap::<LifeComp>(target);
                life.life -= damage.amount;
            }
            // TODO: Things are going to break when trying to get the owner to not be the source. E.g. fungal growth.
            // It targets a unit, but the owner isn't the same.
            EffectToUnit::EFFECT_TO_POINT(to_point) => {
                let (position, owner) = self.c.get2_unwrap::<PositionComp, OwnedComp>(target);
                self.revolve_to_point(data, to_point, &position.pos, owner.owner);
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

