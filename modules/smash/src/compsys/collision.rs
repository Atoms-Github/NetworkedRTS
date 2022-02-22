use crate::*;
use netcode::common::net_game_state::StaticFrameData;

use std::ops::Mul;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CollisionComp {
    pub useless: bool,
}

pub static COLLISION_SYS: System = System{
    run,
    name: "collision"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    let mut deleting = EntStructureChanges::new();
    for (ship_id, ship_collision, ship_position, ship_vel, ship_life, ship_owned,
    ship_size) in
    CompIter6::<CollisionComp, PositionComp, VelocityComp, LifeComp, OwnedComp, SizeComp>::new(c){
        for (rock_id, rock_collision, rock_position, rock_owned, rock_size ) in
        CompIter4::<CollisionComp, PositionComp, OwnedComp, SizeComp>::new(c){

            if ship_id != rock_id && ship_owned.owner != rock_owned.owner{
                let vect_between = (&ship_position.pos - &rock_position.pos).clone();
                let distance = vect_between.norm();
                if distance < (ship_size.size.x + rock_size.size.x) / 2.0{
                    deleting.deleted_entities.push(rock_id);
                    ship_life.life -= 0.5;
                    ship_vel.vel += vect_between.mul(0.3);



                }
            }
        }
        ship_life.life += 0.01;
    }
    deleting.apply(c);

}
