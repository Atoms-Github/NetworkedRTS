use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::event::MouseButton;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
use std::ops::Mul;
use mopa::Any;
use std::ops::Div;
use crate::bibble::effect_resolver::revolver::Revolver;
use crate::bibble::data::data_types::RaceID;
use log::logger;
use walkdir::WalkDir;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ScenePersistent{ // Means keep when scene changes.
    pub keep_alive: bool, // Need to have some sort of size.
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SceneManager{
    pub current: SceneType,
    pub next: SceneType,
    pub completed_rounds: usize,
    pub connected_players: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum SceneType{
    InGame,
    InJigsaw,
    Lobby,
    None,
}

pub static SCENE_SWITCHER_SYS: System = System{
    run,
    name: "scene_switcher"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    let scene = c.get_mut_unwrap::<SceneManager>(SCENE_MAN_ENT_ID);
    // Update current connected player count:
    scene.connected_players = 0;
    for (player_id, player_comp) in CompIter1::<PlayerComp>::new(c){
        if player_comp.connected{
            scene.connected_players += 1;
        }
    }
    // Check for change scene.
    if scene.current != scene.next{
        // Delete all entities (that aren't presistent).
        for entity_id in c.query(vec![]){
            let persist = c.get::<ScenePersistent>(entity_id);
            if persist.is_none() || !persist.unwrap().keep_alive{
                ent_changes.deleted_entities.push(entity_id);
            }
        }
        match scene.next{
            SceneType::InGame => {
                let mut mapname = "NoMapSelected".to_string();
                for (ent_id, map_button_comp) in CompIter1::<MapButtonComp>::new(c){
                    if map_button_comp.selected{
                        mapname = map_button_comp.map.clone();
                    }
                }
                let mut pending_arena_ent = PendingEntity::new_arena(mapname);
                let mut pending_arena = pending_arena_ent.get_mut::<ArenaComp>().unwrap();
                let player_ids = c.query(vec![gett::<PlayerComp>()]);
                for player in player_ids{
                    let player_comp = c.get_unwrap::<PlayerComp>(player);
                    if player_comp.connected{
                        spawn_player_ingame(ent_changes, c, player, player_comp.race, pending_arena);
                    }
                }

                ent_changes.new_entities.push(pending_arena_ent);

                // Reset all cameras so you can see the buttons.
                for (player_id, resources) in CompIter1::<OwnsResourcesComp>::new(c){
                    resources.reset();
                }
            }
            SceneType::Lobby => {
                let game_start_cooldown = {
                    if scene.completed_rounds == 0{
                        20.0
                    }else{
                        5.0
                    }
                };
                // Race picking icons.
                let mut x = 50.0;
                for (race_id, race_mould) in &(0 as GlobalEntityID).get_player_tech_tree(c).races{
                    let new_button = PendingEntity::new_race_selection_button(
                        *race_id, PointFloat::new(x, 200.0), race_mould.icon.clone());
                    ent_changes.new_entities.push(new_button);
                    x += 50.0;
                }
                // Map selection buttons.
                x = 50.0;
                let mut y = 400.0;
                for map_entry in std::fs::read_dir("resources/images/maps").unwrap(){
                    let map_entry = map_entry.unwrap();
                    let new_map_pending = PendingEntity::new_map_selection_button(
                        map_entry.file_name().to_str().unwrap().to_string().clone(),
                    PointFloat::new(x, y), x == 50.0 && y == 400.0);
                    ent_changes.new_entities.push(new_map_pending);
                    x += 250.0;
                    if x > 1500.0{
                        x = 50.0;
                        y += 250.0;
                    }
                }
                // Spawn lobby.
                ent_changes.new_entities.push(PendingEntity::new_lobby(game_start_cooldown));
                // Reset all cameras so you can see the buttons.
                for (player_id, camera) in CompIter1::<CameraComp>::new(c){
                    camera.translation = PointFloat::new(0.0, 0.0);
                }
            }
            SceneType::InJigsaw => {
                let mut filepath = "jigsaws/trees.jpg".to_string();
                let mut lock = crate::rts::game::game_resources::GAME_RESOURCES.lock().unwrap();
                let image = lock.get_image(filepath);

                let last_x = ((image.width() as f32 / JIGSAW_PIECE_SIZE) as i32) - 1;
                let last_y = ((image.height() as f32 / JIGSAW_PIECE_SIZE) as i32) - 1;
                let mut r = StdRng::seed_from_u64(222);

                for x in 0..((image.width() as f32 / JIGSAW_PIECE_SIZE) as i32){
                    for y in 0..((image.height() as f32 / JIGSAW_PIECE_SIZE) as i32){
                        let coords = PointInt::new(x,y);
                        let mut pos = PointFloat::new(x as f32 * JIGSAW_PIECE_SIZE, y as f32 * JIGSAW_PIECE_SIZE);
                        let mut edges = 0;
                        if x == 0 || x == last_x{
                            edges += 1;
                        }
                        if y == 0 || y == last_y{
                            edges += 1;
                        }
                        if edges != 2{ // If not a corner.
                            pos.x = r.gen_range(0.0,image.width() as f32 * 1.0);
                            pos.y = r.gen_range(0.0,image.width() as f32 * 1.0) + JIGSAW_PIECE_SIZE + image.height() as f32;
                        }
                        let pending_piece = PendingEntity::new_jigsaw_piece("trees.jpg".to_string(), coords,
                        pos);

                        ent_changes.new_entities.push(pending_piece);

                    }
                }


            }
            SceneType::None => {}
        }
        scene.current = scene.next.clone();
    }
}

fn spawn_player_ingame(ent_changes: &mut EntStructureChanges, c: &CompStorage, player_id: GlobalEntityID, race: RaceID, arena: &mut ArenaComp){
    let spawn_point = get_player_spawn(player_id, arena, c.find_scene().connected_players as f32);

    let mut revolver = Revolver::new(c);

    let data = player_id.get_player_tech_tree(c);
    let effect = &data.get_race(race).spawn_effect;
    revolver.revolve_to_point(data, effect, &spawn_point, player_id);

    revolver.end().move_into(ent_changes);

    c.get_mut::<PlayerComp>(player_id).unwrap().alive = true;
    c.get_mut::<CameraComp>(player_id).unwrap().translation = spawn_point;
}
fn get_player_spawn(player_id: GlobalEntityID, arena: &mut ArenaComp, max_players: f32) -> PointFloat{
    let radians_round_total  = (std::f64::consts::PI * 2.0) as f32;
    let my_radius_round = (radians_round_total / max_players) * player_id as f32;
    let arena_width = arena.get_length();
    let radius = arena_width as f32 / 3.0;


    let offset_from_centre = PointFloat::new(
        my_radius_round.sin() * radius,
        my_radius_round.cos() * radius
    );

    let centre = PointFloat::new(arena_width as f32 / 2.0, arena_width as f32 / 2.0);

    return centre + offset_from_centre;
}








