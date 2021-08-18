pub use crate::bibble::data::data_types::*;
use crate::rts::GameState;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::LifeComp;
use crate::bibble::effect_resolver::revolver::Revolver;


impl<'a> Revolver<'a>{
    pub fn revolve_to_unit(&mut self, data: &GameData, effect: &EffectToUnit, target: GlobalEntityID){
        match effect{
            EffectToUnit::DAMAGE(damage) => {
                let life = self.c.get1_unwrap::<LifeComp>(target);
                life.life -= damage.amount;
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

