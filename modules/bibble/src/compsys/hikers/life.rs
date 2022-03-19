use crate::*;
use std::ops::Mul;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LifeComp{
    pub life: f32,
    pub max_life: f32,
}

pub static LIFE_SYS: System = System{
    run,
    name: "life"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    for (unit_id, life) in CompIter1::<LifeComp>::new(c) {
        if life.life <= 0.0{
            c.req_delete_entity(unit_id);
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


