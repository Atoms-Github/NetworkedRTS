pub use crate::bibble::data::data_types::*;
use crate::rts::GameState;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::{LifeComp, PositionComp};
use crate::bibble::effect_resolver::revolver::Revolver;
use crate::pub_types::PointFloat;
use crate::ecs::ecs_macros::{CompIter3, CompIter2};


impl<'a> Revolver<'a>{
    pub fn revolve_to_point(&mut self, data: &GameData, effect: &EffectToPoint, target: &PointFloat, owner: GlobalEntityID){
        match effect{
            EffectToPoint::SPAWN_UNIT(unit_id) => {
                let mould = data.units.get(&unit_id).unwrap();
                self.spawn_unit(data, mould, target, owner);
            }
            EffectToPoint::COMPOSITE(effects) => {
                for sub_effect in effects{
                    self.revolve_to_point(data, sub_effect, target, owner);
                }
            }
            EffectToPoint::EFFECT_NEARBY_UNITS(effect, range) => {
                for (unit_id, position, life) in CompIter2::<PositionComp, LifeComp>::new(self.c){
                    if (position.pos.clone() - target).magnitude() < *range{
                        self.revolve_to_unit(data, effect, unit_id);
                    }
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

