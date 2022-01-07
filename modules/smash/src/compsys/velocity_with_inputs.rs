use netcode::common::net_game_state::StaticFrameData;

use crate::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VelocityWithInputsComp {
    pub speed: f32,
}

pub static VELOCITY_WITH_INPUTS_SYS: System = System{
    run,
    name: "velocity_with_inputs"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    for (ent, velocity, velocity_with_inputs, owned) in
    CompIter3::<VelocityComp, VelocityWithInputsComp, OwnedComp>::new(c){
        let my_inputs = &c.get::<InputComp>(owned.owner).unwrap().inputs.primitive;

        let mut speed = velocity_with_inputs.speed;
        if my_inputs.is_keycode_pressed(ggez::input::keyboard::KeyCode::Space){
            speed *= 3.0;
        }
        let mut vector = my_inputs.get_directional() * speed;
        vector.y *= -1.0;
        velocity.vel += vector;
        velocity.vel *= 0.99;
    }
}
