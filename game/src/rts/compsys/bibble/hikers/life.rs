
use crate::rts::compsys::jigsaw::jigsaw_game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::event::MouseButton;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
use std::ops::Mul;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LifeComp{
    pub life: f32,
    pub max_life: f32,
}

pub fn life_sys<C>() -> System<C>{
    System{
        run,
        name: "life"
    }
}
fn run<C>(c: &mut CompStorage<C>, ent_changes: &mut EntStructureChanges<C>, meta: &SimMetadata){
    for (unit_id, life) in CompIter1::<LifeComp>::new(c) {
        if life.life <= 0.0{
            ent_changes.deleted_entities.push(unit_id);
            if let Some(arena) = c.find_arena(){
                if let Some(structure) = c.get::<UnitStructureComp>(unit_id){
                    if let Some(position) = c.get::<PositionComp>(unit_id){
                        if let Some(plots) = arena.get_plot_boxes(position.pos.clone(), structure.structure_info.footprint.clone()){
                            for plot in &plots{
                                arena.set_flooring(plot, structure.structure_info.required_under_material);
                            }
                        }
                    }
                }
            }

        }
    }
}


