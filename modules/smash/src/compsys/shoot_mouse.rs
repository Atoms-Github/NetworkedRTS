use crate::*;
use ggez::winit::event::MouseButton;
use std::ops::Mul;
use becs::pending_entity::PendingEntity;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ShootMouseComp {
    pub time_since_shot: f32
}

pub static SHOOT_MOUSE_SYS: System = System{
    run,
    name: "shoot_mouse"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    let mut changes = EntStructureChanges::new();
    for (id, shoot,owned,position) in CompIter3::<ShootMouseComp, OwnedComp, PositionComp>::new(c){
        let input_state = &c.get::<InputComp>(owned.owner).unwrap().inputs.primitive;

        if shoot.time_since_shot >= 1.0{
            if input_state.get_mouse_pressed(MouseButton::Left){
                let velocity_vec = (input_state.get_mouse_loc() - &position.pos).normalize().mul(6.0);

                
                let mut new_entity = crate::archetypes::new_bullet(owned.owner, position.pos.clone());
                new_entity.set_comp(VelocityComp{ vel: PointFloat::from(velocity_vec) });
                changes.new_entities.push(new_entity);
                shoot.time_since_shot = 0.0;
            }
        }else{
            shoot.time_since_shot += 0.016;
        }
    }
    changes.apply(c);
}