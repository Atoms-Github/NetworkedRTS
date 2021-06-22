
use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::System;
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::MouseButton;

pub struct CollisionComp {
}

pub static COLLISION_SYS: System<GameResources> = System{
    run
};
fn run(res: &GameResources, c: &mut CompStorage){
    for entity_id_ship in c.query(vec![gett::<CollisionComp>(), gett::<LifeComp>(), gett::<OwnedComp>()]){
        for entity_id_rock in c.query(vec![gett::<CollisionComp>(), gett::<PositionComp>(), gett::<OwnedComp>()]){
            let owner1 = c.get::<OwnedComp>(entity_id_ship).unwrap().owner.clone();
            let owner2 = c.get::<OwnedComp>(entity_id_rock).unwrap().owner.clone();
            if entity_id_ship != entity_id_rock && owner1 != owner2{
                let position_rock = c.get::<PositionComp>(entity_id_rock).unwrap().pos.clone();
                let position_ship = c.get::<PositionComp>(entity_id_ship).unwrap().pos.clone();

                let difference = PointFloat::new(position_rock.coords.x - position_ship.coords.x, position_rock.coords.y - position_ship.coords.y);
                if (difference.x * difference.x + difference.y * difference.y) < (70.0 * 70.0){
                    c.get_mut::<LifeComp>(entity_id_ship).unwrap().life -= 0.5;
                }
            }
        }
        c.get_mut::<LifeComp>(entity_id_ship).unwrap().life += 0.01;

    }

}