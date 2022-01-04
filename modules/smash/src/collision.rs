
use crate::rts::compsys::jigsaw::jigsaw_game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::event::MouseButton;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CollisionComp {
    pub useless: bool,
}

pub static COLLISION_SYS: System = System{
    run,
    name: "collision"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    for (ship_id, ship_collision, ship_position, ship_life, ship_owned) in
    CompIter4::<CollisionComp, PositionComp, LifeComp, OwnedComp>::new(c){
        for (rock_id, rock_collision, rock_position, rock_owned ) in
        CompIter3::<CollisionComp, PositionComp, OwnedComp>::new(c){

            if ship_id != rock_id && ship_owned.owner != rock_owned.owner{
                let distance = (&rock_position.pos - &ship_position.pos).norm();
                if distance < 70.0{
                    ship_life.life -= 0.5;
                }
            }
        }
        ship_life.life += 0.01;
    }

}