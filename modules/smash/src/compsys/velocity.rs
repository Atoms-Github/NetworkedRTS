

use crate::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VelocityComp {
    pub vel: PointFloat,
}

pub static VELOCITY_SYS: System = System{
    run,
    name: "velocity"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    for (entity_id, velocity, position) in CompIter2::<VelocityComp, PositionComp>::new(c){
        position.pos += &velocity.vel;
    }
}