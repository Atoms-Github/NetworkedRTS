pub use crate::bibble::data::data_types::*;
use crate::rts::GameState;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::LifeComp;
use crate::bibble::effect_resolver::revolver::Revolver;
use crate::pub_types::PointFloat;


impl<'a> Revolver<'a>{
    pub fn resolve_tp(&mut self, data: &GameData, effect: EffectToPoint, target: PointFloat, owner: GlobalEntityID){
        match effect{
            EffectToPoint::SPAWN_UNIT(unit_id) => {
                let mould = data.units.get(&unit_id).unwrap();
                self.spawn_unit(data, mould, target, owner);
            }
            EffectToPoint::COMPOSITE(effects) => {
                for sub_effect in effects{
                    self.resolve_tp(data, sub_effect, target.clone(), owner);
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

