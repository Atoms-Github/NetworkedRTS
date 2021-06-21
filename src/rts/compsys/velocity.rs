use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::System;
use crate::ecs::comp_store::CompStorage;

pub struct VelocityComp {
    pub vel: PointFloat,
}

pub static VELOCITY_SYS: System<GameResources> = System{
    run
};
fn run(res: &GameResources, ecs: &mut CompStorage){
    for entity_id in ecs.query(vec![gett::<VelocityComp>(), gett::<PositionComp>()]){
        // ***noice*** /s
        ecs.get_mut::<PositionComp>(entity_id).unwrap().pos.x += ecs.get::<VelocityComp>(entity_id).unwrap().vel.x;
        ecs.get_mut::<PositionComp>(entity_id).unwrap().pos.y += ecs.get::<VelocityComp>(entity_id).unwrap().vel.y;
    }
}