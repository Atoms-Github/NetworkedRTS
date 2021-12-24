pub use crate::bibble::data::data_types::*;
use crate::rts::GameStateJigsaw;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::{LifeComp, PositionComp, MyCompStorage};
use crate::bibble::effect_resolver::revolver::Revolver;
use crate::pub_types::PointFloat;
use crate::ecs::ecs_macros::{CompIter3, CompIter2};
use crate::utils;
use crate::rts::compsys::the_map::arena::PlotFlooring;

impl<'a, C> Revolver<'a, C>{
    pub fn revolve_to_point(&mut self, data: &GameData, effect: &EffectToPoint, target: &PointFloat, owner: GlobalEntityID){
        match effect{
            EffectToPoint::SPAWN_UNIT(unit_id) => {
                let mould = data.units.get(&unit_id).unwrap();
                self.spawn_unit(data, mould, target, owner);
            }
            EffectToPoint::BUILD_BUILDING(unit_id) => {
                let mould = data.units.get(&unit_id).unwrap();
                let structure = crate::utils::unwrap!(UnitFlavour::STRUCTURE, &mould.unit_flavour);
                let arena = self.c.find_arena().unwrap();
                if let Some(plots) = arena.get_plot_boxes(target.clone(), structure.footprint.clone()){
                    let mut good_spawn = true;
                    for plot in &plots{
                        if arena.get_flooring(plot) != structure.required_under_material{
                            good_spawn = false;
                        }
                    }
                    if good_spawn{
                        for plot in &plots{
                            arena.set_flooring(plot, PlotFlooring::STRUCTURE);
                        }
                        self.spawn_unit(data, mould, target, owner);
                    }

                }
            }
            EffectToPoint::COMPOSITE(effects) => {
                for sub_effect in effects{
                    self.revolve_to_point(data, sub_effect, target, owner);
                }
            }
            EffectToPoint::EFFECT_NEARBY_UNITS(effect, range) => {
                for (unit_id, position, life) in CompIter2::<C, PositionComp, LifeComp>::new(self.c){
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

