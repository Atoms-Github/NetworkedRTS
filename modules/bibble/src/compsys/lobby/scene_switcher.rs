use crate::*;
use std::ops::Mul;
use std::ops::Div;
use log::logger;




#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SceneManager{
    pub current: RtsSceneType,
    pub next: RtsSceneType,
    pub completed_rounds: usize,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum RtsSceneType {
    InGame,
    Lobby,
    None,
}

pub static SCENE_SWITCHER_SYS: System = System{
    run,
    name: "scene_switcher"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    let scene = c.get_mut_unwrap::<SceneManager>(OVERSEER_ENT_ID);

    // Check for change scene.
    if scene.current != scene.next{
        // Delete all entities (that aren't presistent).
        ScenePersistent::delete_all_non_persistent(c);

        match scene.next{
            RtsSceneType::InGame => {
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
            RtsSceneType::Lobby => {
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
                let size = 150.0;
                x = size;
                let mut y = size + 150.0;
                for map_entry in std::fs::read_dir("../../../../../../resources/images/jigsaws").unwrap(){
                    let map_entry = map_entry.unwrap();
                    let new_map_pending = PendingEntity::new_map_selection_button(
                        map_entry.file_name().to_str().unwrap().to_string().clone(),
                        PointFloat::new(x, y), x == size && y == size + 150.0, size);
                    ent_changes.new_entities.push(new_map_pending);
                    x += size;
                    if x > 1500.0{
                        x = size;
                        y += size;
                    }
                }
                // Spawn lobby.
                ent_changes.new_entities.push(PendingEntity::new_lobby(game_start_cooldown));
                // Reset all cameras so you can see the buttons.
                for (player_id, camera) in CompIter1::<CameraComp>::new(c){
                    camera.translation = PointFloat::new(0.0, 0.0);
                }
            }
            RtsSceneType::None => {}
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








