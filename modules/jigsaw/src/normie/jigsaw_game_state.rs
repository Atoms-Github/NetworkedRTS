use crate::*;
use netcode::*;
use ggez::{*};
use std::sync::Arc;
use ggez::graphics::{DrawParam, Text};
use nalgebra::Point2;
pub use crate::utils::gett;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::Rng;
use bib_utils::debug_timer::DT;
use netcode::common::net_game_state::GameState;

pub const MAX_PLAYERS : usize = 16;

#[derive(Clone, Serialize, Deserialize, Hash, Debug)]
pub struct GameStateJigsaw {
    #[serde(deserialize_with = "state_deserialize")]
    ecs: SuperbEcs,
}



impl Default for GameStateJigsaw {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState for GameStateJigsaw {
    fn new() -> Self {
        Self{
            ecs: SuperbEcs::new(get_config()),
        }
    }
    fn init(&mut self){
        self.ecs.c.req_create_entity(new_scene_manager());
        //Reserve entity ids 1 to 9ish so player ID and entity IDs match up.
        for player_index in 1..9{
            let mut pending = archetypes::new_player(player_index as GlobalEntityID);
            self.ecs.c.req_create_entity(pending);
        }
    }
    fn simulate_tick(&mut self, stat: &StaticFrameData) {
        self.ecs.sim_systems(&stat);

        self.ecs.c.flush_ent_changes();
    }

    fn render(&mut self, ctx: &mut Context, player_id: PlayerID, res: &mut GgEzResources){
        let mut batcher = CoolBatcher::new();

        pcommon::simple_render(&mut batcher, &mut self.ecs, player_id as GlobalEntityID);
        jigsaw_render(&mut batcher, &mut self.ecs, player_id as GlobalEntityID);
        batcher.gogo_draw(ctx, &mut res.render);
    }
    fn gen_resources(ctx: &mut ggez::Context) -> Self::Resources {
        let mut resources = Self::Resources::default();

        return resources;
    }

    type Resources = GgEzResources;
}
fn get_config() -> EcsConfig{
    EcsConfig{
        functions: {
            let mut map = FunctionMap::default();
            map.register_type::<PositionComp>();
            map.register_type::<RadiusComp>();
            map.register_type::<SizeComp>();
            map.register_type::<CameraComp>();
            map.register_type::<InputComp>();
            map.register_type::<OwnedComp>();
            map.register_type::<OwnsResourcesComp<JigsawPlayerProperty>>();
            map.register_type::<PlayerComp>();
            map.register_type::<RenderComp>();
            map.register_type::<ClickableComp>();
            map.register_type::<UIComp>();
            map.register_type::<CursorComp>();
            map.register_type::<UninteractableComp>();
            map.register_type::<JigsawButtonComp>();
            map.register_type::<JigsawMatComp>();
            map.register_type::<JigsawPieceComp>();
            map.register_type::<JigsawPlayerComp>();
            map.register_type::<LobbyManagerComp>();
            map.register_type::<JigsawSceneManager>();
            map.register_type::<SimpleViewerComp>();
            map.register_type::<ScenePersistent>();
            map
        },
        systems: vec![
            INPUT_PREPROC.clone(),
            PLAYER_CONNECT.clone(),
            PLAYER_DISCONNECT.clone(),
            BUTTON_SYS.clone(),
            PERFORMANCE_MAP.clone(),
            CAMERA_SYS.clone(),
            JIGSAW_BUTTON_SYS.clone(),
            JIGSAW_MAT_SYS.clone(),
            JIGSAW_PIECE_SYS.clone(),
            JIGSAW_PLAYER_SYS.clone(),
            LOBBY_SYS.clone(),
            JIGSAW_SCENE_SWITCHER_SYS.clone(),
            CURSOR_SYS.clone(),
            UI_SYS.clone(),
        ]
    }
}

fn state_deserialize<'de, D>(deserializer: D) -> Result<SuperbEcs, D::Error> where D: Deserializer<'de> {
    match SuperbEcs::deserialize(deserializer){
        Ok(mut ecs) => {
            ecs.post_deserialize(get_config());
            Ok(ecs)
        }
        Err(err) => {Err(err)}
    }
}