use crate::*;
use std::ops::Mul;
use std::ops::Div;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct HikerCollisionComp {
    pub radius: f32,
    pub fly: bool,
}
pub static HIKER_COLLISION_SYS: System = System{
    run,
    name: "hiker_collision"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    do_bops(c);
    // Do walls:
    if let Some(arena) = c.find_arena(){
        for (unit_id, position, life, hiker_collision) in
        CompIter3::<PositionComp, LifeComp, HikerCollisionComp>::new(c) {
            if hiker_collision.fly{
                continue;
            }
            let unit_box_x = (position.pos.x / ARENA_PLOT_SIZE) as i32;
            let unit_box_y = (position.pos.y / ARENA_PLOT_SIZE) as i32;
            for box_x in unit_box_x - 1..unit_box_x + 2{
                for box_y in unit_box_y - 1..unit_box_y + 2{
                    if !arena.is_box_walkable(&GridBox::new(box_x, box_y)){
                        let closest_x_to_circle = position.pos.x.clamp((box_x * ARENA_PLOT_SIZE as i32) as f32, ((box_x + 1) * ARENA_PLOT_SIZE as i32) as f32);
                        let closest_y_to_circle = position.pos.y.clamp((box_y * ARENA_PLOT_SIZE as i32) as f32, ((box_y + 1) * ARENA_PLOT_SIZE as i32) as f32);
                        let closest_point_on_box = PointFloat::new(closest_x_to_circle, closest_y_to_circle);
                        let distance_apart = (closest_point_on_box.clone() - &position.pos).magnitude();
                        let minimum_distance_apart = hiker_collision.radius;
                        if distance_apart < minimum_distance_apart{
                            apply_bop(minimum_distance_apart - distance_apart, &mut position.pos, &closest_point_on_box);
                        }
                    }
                }
            }
        }
    }
}

fn do_bops(c: &mut CompStorage) {
    let unit_ids = c.query(vec![
        crate::utils::gett::<HikerCollisionComp>(),
        crate::utils::gett::<HikerComp>(),
        crate::utils::gett::<PositionComp>(),
    ]);
    let mut comps = vec![];

    for unit_id in &unit_ids {
        comps.push((
            *unit_id,
            c.get_mut::<HikerCollisionComp>(*unit_id).unwrap(),
            c.get_mut::<HikerComp>(*unit_id).unwrap(),
            c.get_mut::<PositionComp>(*unit_id).unwrap()
        ));
    }
    for (unit_id_1, hiker_collision_1, hiker_comp_1, position_1) in &comps {
        for (unit_id_2, hiker_collision_2, hiker_comp_2, position_2) in &comps {
            if unit_id_1 != unit_id_2 {
                let actual_distance_squared = (position_1.pos.x - position_2.pos.x).powi(2) + (position_1.pos.y - position_2.pos.y).powi(2);
                let min_distance = hiker_collision_1.radius + hiker_collision_2.radius;
                if actual_distance_squared < min_distance.powi(2) {
                    let distance_too_close = min_distance - actual_distance_squared.sqrt();
                    const IMPORTANTER_ONE_BOP_FRACTION: f32 = 0.25;
                    let bop_fraction_for_1 = {
                        if hiker_comp_1.quest_importance == hiker_comp_2.quest_importance {
                            0.5
                        } else if hiker_comp_1.quest_importance > hiker_comp_2.quest_importance {
                            IMPORTANTER_ONE_BOP_FRACTION
                        } else {
                            1.0 - IMPORTANTER_ONE_BOP_FRACTION
                        }
                    };
                    let bop_dist_1 = bop_fraction_for_1 * distance_too_close;
                    let bop_dist_2 = (1.0 - bop_fraction_for_1) * distance_too_close;
                    apply_bop(bop_dist_1, unsafe { game::utils::unsafe_const_cheat(&position_1.pos) }, &position_2.pos);
                    apply_bop(bop_dist_2, unsafe { game::utils::unsafe_const_cheat(&position_2.pos) }, &position_1.pos);
                }
            }
        }
    }
}

fn apply_bop(bop_dist: f32, boppee: &mut PointFloat, bopper: &PointFloat){
    let pos_diff = boppee.clone() - bopper;
    let safe_diff = {
        if pos_diff.magnitude_squared() > 0.01{
            pos_diff
        }else{
            PointFloat::new(1.0,0.0)
        }
    };
    let move_dist = safe_diff.normalize().mul(bop_dist);
    *boppee += move_dist;
}














//







/* Original (slow) version. (1.1ms for 220 entities)

for (unit_id_1, hiker_collision_1, hiker_comp_1, position_1) in CompIter3::<>::new(c) {
        for (unit_id_2, hiker_collision_2, hiker_comp_2, position_2) in CompIter3::<HikerCollisionComp, HikerComp, PositionComp>::new(c) {
            if unit_id_1 != unit_id_2 && position_1.pos.x >= 1000000000.0{
                let actual_distance_squared =  (position_1.pos.x - position_2.pos.x).powi(2) + (position_1.pos.y - position_2.pos.y).powi(2);
                let min_distance = hiker_collision_1.radius + hiker_collision_2.radius;
                if actual_distance_squared < min_distance.powi(2) {
                    let distance_too_close = min_distance - actual_distance_squared.sqrt();
                    const IMPORTANTER_ONE_BOP_FRACTION : f32 = 0.25;
                    let bop_fraction_for_1 = {
                        if hiker_comp_1.quest_importance == hiker_comp_2.quest_importance{
                            0.5
                        }else if hiker_comp_1.quest_importance > hiker_comp_2.quest_importance{
                            IMPORTANTER_ONE_BOP_FRACTION
                        }else{
                            1.0 - IMPORTANTER_ONE_BOP_FRACTION
                        }
                    };
                    let bop_dist_1 = bop_fraction_for_1 * distance_too_close;
                    let bop_dist_2 = (1.0 - bop_fraction_for_1) * distance_too_close;
                    apply_bop(bop_dist_1, position_1, position_2);
                    apply_bop(bop_dist_2, position_2, position_1);
                }
            }
        }
    }

 */

