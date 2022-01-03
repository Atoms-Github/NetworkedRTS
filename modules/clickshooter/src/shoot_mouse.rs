use crate::rts::compsys::jigsaw::jigsaw_game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::{PointFloat, RenderResourcesPtr};
use crate::ecs::superb_ecs::*;
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::event::MouseButton;
use std::ops::Mul;
use serde_closure::internal::std::borrow::Cow::Owned;
use crate::ecs::ecs_macros::CompIter3;
use serde_closure::internal::std::future::Pending;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ShootMouseComp {
    pub time_since_shot: f32
}

pub static SHOOT_MOUSE_SYS: System = System{
    run,
    name: "shoot_mouse"
};
// Macros: eget!(); to get variable nubmer of compoentns.
// query_id!();
// query!();
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    // for (id, shoot,owned,position) in CompIter3::<ShootMouseComp, OwnedComp, PositionComp>::new(c){
    //     let input_state = &c.get::<InputComp>(owned.owner).unwrap().inputs.primitive;
    //
    //     if shoot.time_since_shot >= 1.0{
    //
    //         if input_state.get_mouse_pressed(MouseButton::Left){
    //             let velocity_vec = (input_state.get_mouse_loc() - &position.pos).normalize().mul(6.0);
    //             let mut new_entity = PendingEntity::new_bullet(owned.owner, position.pos.clone());
    //             new_entity.set_comp(VelocityComp{ vel: PointFloat::from(velocity_vec) });
    //             ent_changes.new_entities.push(new_entity);
    //
    //             shoot.time_since_shot = 0.0;
    //         }
    //     }else{
    //         shoot.time_since_shot += 0.016;
    //     }
    // }
}