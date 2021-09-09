use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::MouseButton;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
use std::ops::Mul;
use mopa::Any;
use std::ops::Div;
use crate::bibble::effect_resolver::revolver::Revolver;
use crate::bibble::data::data_types::RaceID;
use log::logger;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ScenePersistent{ // Means keep when scene changes.
    pub keep_alive: bool, // Need to have some sort of size.
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SceneManager{
    pub current: SceneType,
    pub next: SceneType,
    pub completed_rounds: usize,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum SceneType{
    InGame,
    Lobby,
    None,
}

pub static SCENE_SWITCHER_SYS: System = System{
    run,
    name: "scene_switcher"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges) {
    let scene = c.get_mut_unwrap::<SceneManager>(SCENE_MAN_ENT_ID);
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
                let mut pending_arena_ent = PendingEntity::new_arena();
                let mut pending_arena = pending_arena_ent.get_mut::<ArenaComp>().unwrap();
                let player_ids = c.query(vec![gett::<PlayerComp>()]);
                for player in player_ids{
                    spawn_player_ingame(ent_changes, c, player, c.get_unwrap::<PlayerComp>(player).race, pending_arena);
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
                let player_one : GlobalEntityID = 0;
                let mut x = 50.0;
                for (race_id, race_mould) in &player_one.get_player_tech_tree(c).races{
                    ent_changes.new_entities.push(PendingEntity::new_race_selection_button(*race_id,
                    PointFloat::new(x, 200.0)));
                    x += 50.0;
                }
                ent_changes.new_entities.push(PendingEntity::new_lobby(game_start_cooldown));
                // Reset all cameras so you can see the buttons.
                for (player_id, camera) in CompIter1::<CameraComp>::new(c){
                    camera.translation = PointFloat::new(0.0, 0.0);
                }
            }
            SceneType::None => {}
        }
        scene.current = scene.next.clone();
    }
}

fn spawn_player_ingame(ent_changes: &mut EntStructureChanges, c: &CompStorage, player_id: GlobalEntityID, race: RaceID, arena: &mut ArenaComp){
    let spawn_point = get_player_spawn(player_id, arena);

    let mut revolver = Revolver::new(c);

    let data = player_id.get_player_tech_tree(c);
    let effect = &data.get_race(race).spawn_effect;
    revolver.revolve_to_point(data, effect, &spawn_point, player_id);

    revolver.end().move_into(ent_changes);

    c.get_mut::<PlayerComp>(player_id).unwrap().alive = true;
    c.get_mut::<CameraComp>(player_id).unwrap().translation = spawn_point;
}
fn get_player_spawn(player_id: GlobalEntityID, arena: &mut ArenaComp) -> PointFloat{
    let radians_round_total  = (std::f64::consts::PI * 2.0) as f32;
    let my_radius_round = (radians_round_total / MAX_PLAYERS as f32) * player_id as f32;
    let arena_width = arena.get_length();
    let radius = arena_width as f32 / 3.0;


    let offset_from_centre = PointFloat::new(
        my_radius_round.sin() * radius,
        my_radius_round.cos() * radius
    );

    let centre = PointFloat::new(arena_width as f32 / 2.0, arena_width as f32 / 2.0);

    return centre + offset_from_centre;
}








