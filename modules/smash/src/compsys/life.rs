use crate::*;
use netcode::common::net_game_state::StaticFrameData;


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
    let mut changes = EntStructureChanges::new();
    for (unit_id, life) in CompIter1::<LifeComp>::new(c) {
        if life.life <= 0.0{
            changes.deleted_entities.push(unit_id);
        }
    }
    changes.apply(c);
}


