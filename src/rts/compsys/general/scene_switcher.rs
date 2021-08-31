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


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ScenePersistent{ // Means keep when scene changes.
    pub keep_alive: bool, // Need to have some sort of size.
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SceneManager{
    pub current: SceneType,
    pub next: SceneType,
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
    let scene_man = c.get_unwrap::<SceneManager>(SCENE_ENT_ID);
    if scene_man.current != scene_man.next{
        // Delete all entities (that aren't presistent).
        for entity_id in c.query(vec![]){
            let persist = c.get::<ScenePersistent>(entity_id);
            if persist.is_none() || !persist.unwrap().keep_alive{
                ent_changes.deleted_entities.push(entity_id);
            }
        }
        match scene_man.next{
            SceneType::InGame => {
                let mut pending_arena_ent = PendingEntity::new_arena();
                let mut pending_arena = pending_arena_ent.get_mut::<ArenaComp>().unwrap();
                let player_ids = c.query(vec![gett::<PlayerComp>()]);
                for player in player_ids{
                    spawn_player_ingame(c, player, RaceID::ROBOTS, pending_arena);
                }

                ent_changes.new_entities.push(pending_arena_ent);
            }
            SceneType::Lobby => {

            }
            SceneType::None => {}
        }

    }
}

fn spawn_player_ingame(c: &mut CompStorage, player_id: GlobalEntityID, race: RaceID, arena: &mut ArenaComp){
    let spawn_point = get_player_spawn(c, player_id, arena);

    let mut revolver = Revolver::new(c);

    let data = player_id.get_player_tech_tree(c);
    let effect = &data.get_race(race).spawn_effect;
    revolver.revolve_to_point(data, effect, &spawn_point, player_id);

    revolver.end().apply(c);

    c.get_mut::<PlayerComp>(player_id).unwrap().alive = true;
    c.get_mut::<CameraComp>(player_id).unwrap().translation = spawn_point;
}
fn get_player_spawn(c: &mut CompStorage, player_id: GlobalEntityID, arena: &mut ArenaComp) -> PointFloat{
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








