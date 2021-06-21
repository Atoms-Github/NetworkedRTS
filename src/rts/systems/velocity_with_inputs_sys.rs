use crate::ecs::superb_ecs::*;
use crate::rts::game::game_state::*;
use crate::ecs::comp_store::CompStorage;
use crate::pub_types::PointFloat;
use crate::rts::systems::velocity_sys::VelocityComp;
use crate::rts::comps::owner_comp::OwnedComp;
use crate::rts::comps::player_comp::PlayerComp;


pub struct VelocityWithInputsComp {
    pub speed: f32,
}

pub static VELOCITY_SYSTEM : System<GameResources> = System{
    run
};
fn run(res: &GameResources, ecs: &mut CompStorage){
    for entity_id in ecs.query(vec![gett::<VelocityComp>(), gett::<VelocityWithInputsComp>(), gett::<OwnedComp>()]){
        let owner_id = ecs.get::<OwnedComp>(entity_id).unwrap().owner;
        let my_inputs = ecs.get::<PlayerComp>(owner_id).unwrap().inputs.clone();

        let (directional_x, directional_y) = my_inputs.get_directional();

        let mut my_speed = ecs.get::<VelocityWithInputsComp>(entity_id).unwrap().speed;

        if my_inputs.is_keycode_pressed(ggez::input::keyboard::KeyCode::Space){
            my_speed *= 3.0;
        }
        let velocity = ecs.get_mut::<VelocityComp>(entity_id).unwrap();
        velocity.vel.x = my_speed * directional_x;
        velocity.vel.y = my_speed * -directional_y;
    }
}
