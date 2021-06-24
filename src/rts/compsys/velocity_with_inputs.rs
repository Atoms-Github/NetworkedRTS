use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::ecs::comp_store::CompStorage;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use std::ops::Mul;

pub struct VelocityWithInputsComp {
    pub speed: f32,
}

pub static VELOCITY_WITH_INPUTS_SYS: System<GameResources> = System{
    run
};
fn run(res: &GameResources, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (ent, velocity, velocity_with_inputs, owned) in
    CompIter3::<VelocityComp, VelocityWithInputsComp, OwnedComp>::new(c){
        let my_inputs = &c.get::<PlayerComp>(owned.owner).unwrap().inputs;

        let mut speed = velocity_with_inputs.speed;
        if my_inputs.is_keycode_pressed(ggez::input::keyboard::KeyCode::Space){
            speed *= 3.0;
        }
        let mut vector = my_inputs.get_directional() * speed;
        vector.y *= -1.0;
        velocity.vel = nalgebra::Point2::from(vector);
    }
}
