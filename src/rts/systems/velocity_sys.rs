use crate::ecs::superb_ecs::*;
use crate::rts::game::game_state::*;
use crate::ecs::comp_store::CompStorage;
use crate::pub_types::PointFloat;

pub struct VelocityComp {
    pub vel: PointFloat,
}

pub static VELOCITY_SYSTEM : System<GameResources> = System{
    run
};
fn run(res: &GameResources, ecs: &mut CompStorage){
    // for entity_id in ecs.query(vec![crate::utils::get_type_id::<VelocityComp>(), crate::utils::get_type_id::<PositionComp>()]){
    //     // ***noice*** /s
    //     ecs.get_mut::<PositionComp>(entity_id).unwrap().pos.x += ecs.get::<VelocityComp>(entity_id).unwrap().vel.x;
    //     ecs.get_mut::<PositionComp>(entity_id).unwrap().pos.y += ecs.get::<VelocityComp>(entity_id).unwrap().vel.y;
    // }
}