use crate::*;
use std::ops::Mul;
use std::ops::Div;
use log::logger;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};




#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct JigsawSceneManager {
    pub current: JigsawSceneType,
    pub next: JigsawSceneType,
    pub completed_rounds: usize,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum JigsawSceneType {
    InJigsaw,
    Lobby,
    None,
}

pub static JIGSAW_SCENE_SWITCHER_SYS: System = System{
    run,
    name: "scene_switcher"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    let scene = c.get_mut_unwrap::<JigsawSceneManager>(OVERSEER_ENT_ID);

    // Check for change scene.
    if scene.current != scene.next{
        // Delete all entities (that aren't presistent).
        ScenePersistent::delete_all_non_persistent(c);

        match scene.next{
            JigsawSceneType::Lobby => {
                let mut lock = pcommon::LOGIC_RESOURCES.lock().unwrap();

                // Map selection buttons.
                let size = 150.0;
                let mut x = size;
                let mut y = size + 150.0;
                for map_entry in lock.iter_directory("resources/images/jigsaws".to_string()){
                    let new_map_pending = new_jigsaw_selection_button(
                        format!("jigsaws/{}", map_entry),
                        PointFloat::new(x, y), x == size && y == size + 150.0, size);
                    c.req_create_entity(new_map_pending);
                    x += size;
                    if x > 1500.0{
                        x = size;
                        y += size;
                    }
                }
                c.req_create_entity(new_lobby());
                // Spawn lobby.
                // Reset all cameras so you can see the buttons.
                for (player_id, camera) in CompIter1::<CameraComp>::new(c){
                    camera.translation = PointFloat::new(0.0, 0.0); // TODO0: Needed?
                }
            }
            JigsawSceneType::InJigsaw => {
                let mut mapname = "trees.jpg".to_string();
                for (ent_id, jigsaw_button_comp) in CompIter1::<JigsawButtonComp>::new(c){
                    if jigsaw_button_comp.selected{
                        mapname = jigsaw_button_comp.map.clone();
                    }
                }
                let mut lock = pcommon::LOGIC_RESOURCES.lock().unwrap();
                let image = lock.get_image(mapname.clone());

                let last_x = ((image.width() as f32 / JIGSAW_PIECE_SIZE) as i32) - 1;
                let last_y = ((image.height() as f32 / JIGSAW_PIECE_SIZE) as i32) - 1;
                let mut r = StdRng::seed_from_u64(222);
                let mut piece_index = 0;
                for x in 0..((image.width() as f32 / JIGSAW_PIECE_SIZE) as i32){
                    for y in 0..((image.height() as f32 / JIGSAW_PIECE_SIZE) as i32){
                        let coords = PointInt::new(x,y);
                        let mut pos ;//= PointFloat::new(x as f32 * JIGSAW_PIECE_SIZE,y as f32 * JIGSAW_PIECE_SIZE);
                        let border_width = 0.3;
                        let mut attempted_location = PointFloat::new(r.gen_range(image.width() as f32 * -border_width,image.width() as f32 * (1.0 + border_width)),
                                                                     r.gen_range(image.height() as f32 * -border_width,image.height() as f32 * (1.0 + border_width)));
                        let jigsaw_rect = ggez::graphics::Rect::new(0.0,0.0,image.width() as f32, image.height() as f32);
                        for i in 0..500{
                            if jigsaw_rect.contains(attempted_location.to_point()){
                                attempted_location = PointFloat::new(r.gen_range(image.width() as f32 * -border_width,image.width() as f32 * (1.0 + border_width)),
                                                                     r.gen_range(image.height() as f32 * -border_width,image.height() as f32 * (1.0 + border_width)));
                            }else{
                                break;
                            }
                        }
                        pos = attempted_location;

                        c.req_create_entity(new_jigsaw_piece(mapname.clone(), coords,pos,
                        JZValue::JigsawPieceHeld.g() + piece_index));
                        piece_index += 1;
                        println!("Spawning index: {:?}", JZValue::JigsawPieceHeld.g() + piece_index);
                    }
                }
                c.req_create_entity(new_jigsaw_mat(mapname.clone(),
                                                   PointFloat::new((last_x + 1) as f32 * JIGSAW_PIECE_SIZE,
                                                                   (last_y + 1) as f32 * JIGSAW_PIECE_SIZE),
                piece_index));
                println!("Loading jigsaw with {} pieces", piece_index);


            }
            JigsawSceneType::None => {}
        }
        scene.current = scene.next.clone();
    }
}






