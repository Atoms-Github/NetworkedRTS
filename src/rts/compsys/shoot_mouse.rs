use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::MouseButton;
use std::ops::Mul;
use serde_closure::internal::std::borrow::Cow::Owned;
use crate::ecs::ecs_macros::CompIter3;

pub struct ShootMouseComp {
    pub time_since_shot: f32
}

pub static SHOOT_MOUSE_SYS: System<GameResources> = System{
    run
};
// Macros: eget!(); to get variable nubmer of compoentns.
// query_id!();
// query!();
fn run(res: &GameResources, c: &mut CompStorage) {
    let mut entities = vec![];
    for (id, shoot,owned,position) in CompIter3::<ShootMouseComp, OwnedComp, PositionComp>::new(c){
        let input_state = &c.get::<PlayerComp>(owned.owner).unwrap().inputs;

        if shoot.time_since_shot >= 1.0{
            if input_state.get_mouse_pressed(MouseButton::Left){
                let velocity_vec = (input_state.get_mouse_loc().clone().coords - position.pos.clone().coords).normalize().mul(6.0);
                let mut new_entity = PendingEntity::new();
                new_entity.add_comp(RenderComp{ colour: (100,50,50) });
                new_entity.add_comp(PositionComp{ pos: position.pos.clone()});
                new_entity.add_comp(CollisionComp{  });
                new_entity.add_comp(VelocityComp{ vel: PointFloat::from(velocity_vec) });
                new_entity.add_comp( OwnedComp { owner: owned.owner });
                entities.push(new_entity);


                shoot.time_since_shot = 0.0;
            }
        }else{
            shoot.time_since_shot += 0.016;
        }
    }
    for entity in entities{
        c.create_entity(entity);
    }
    // for entity_id in c.query(vec![gett::<ShootMouseComp>(), gett::<OwnedComp>(), gett::<PositionComp>()]){
    //     let player_id = c.get::<OwnedComp>(entity_id).unwrap().owner;
    //     let position = c.get::<PositionComp>(entity_id).unwrap().pos.clone();
    //     let input_state = c.get::<PlayerComp>(player_id).unwrap().inputs.clone();
    //     let target = input_state.get_mouse_loc().clone();
    //
    //     if c.get::<ShootMouseComp>(entity_id).unwrap().time_since_shot >= 1.0{
    //         if input_state.get_mouse_pressed(MouseButton::Left){
    //
    //             let velocity_vec = (target.coords - position.clone().coords).normalize().mul(6.0);
    //             let mut new_entity = PendingEntity::new();
    //             new_entity.add_comp(RenderComp{ colour: (100,50,50) });
    //             new_entity.add_comp(PositionComp{ pos: position });
    //             new_entity.add_comp(CollisionComp{  });
    //             new_entity.add_comp(VelocityComp{ vel: PointFloat::from(velocity_vec) });
    //             new_entity.add_comp( OwnedComp { owner: player_id });
    //             c.create_entity(new_entity);
    //
    //             c.get_mut::<ShootMouseComp>(entity_id).unwrap().time_since_shot = 0.0;
    //         }
    //     }else{
    //         c.get_mut::<ShootMouseComp>(entity_id).unwrap().time_since_shot += 0.016;
    //     }
    // }
}