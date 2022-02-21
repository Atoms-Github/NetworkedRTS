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
use netcode::common::net_game_state::GameState;

pub const MAX_PLAYERS : usize = 16;
pub const SCENE_MAN_ENT_ID: GlobalEntityID = MAX_PLAYERS;

pub type UsingRenderResources = Arc<GgEzRenderResources>;



#[derive(Clone, Serialize, Deserialize, Hash, Debug)]
pub struct GameStateJigsaw {
    #[serde(deserialize_with = "state_deserialize")]
    ecs: ActiveEcs,
}



impl Default for GameStateJigsaw {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState for GameStateJigsaw {
    fn new() -> Self {
        Self{
            ecs: ActiveEcs::new(get_config()),
        }
    }
    fn init(&mut self){
        //Reserve entity ids 0 to 8ish so player ID and entity IDs match up.
        for player_index in 0..MAX_PLAYERS{
            let mut pending = PendingEntity::new_player(player_index as GlobalEntityID);
            assert_eq!(player_index, self.ecs.c.create_entity(pending))
        }
        assert_eq!(self.ecs.c.create_entity(PendingEntity::new_scene_manager()), SCENE_MAN_ENT_ID)
    }
    fn player_connects(&mut self, player_id: PlayerID, username: String, color: Shade){
        let player_ent_id = player_id as GlobalEntityID;

        self.ecs.c.get_mut::<PlayerComp>(player_ent_id).unwrap().name = username;
        self.ecs.c.get_mut::<PlayerComp>(player_ent_id).unwrap().color = color;
        self.ecs.c.get_mut::<PlayerComp>(player_ent_id).unwrap().connected = true;


        let cursor = PendingEntity::new_cursor(player_ent_id, color);
        self.ecs.c.create_entity(cursor);
    }
    fn player_disconnects(&mut self, player_id: PlayerID){
        self.ecs.c.get_mut::<PlayerComp>(player_id as GlobalEntityID).unwrap().connected = false;
        let mut my_cursor = None;
        for (cursor_id, cursor_comp, position) in CompIter2::<CursorComp, PositionComp>::new(&self.ecs.c){
            if cursor_comp.player == player_id as GlobalEntityID{
                my_cursor = Some(cursor_id);
            }
        }
        if let Some(cursor) = my_cursor{
            self.ecs.c.delete_entity(cursor);
        }
    }
    fn simulate_tick(&mut self, inputs: PlayerInputs, sim_meta: &SimMetadata){
        for (player_id, input_state) in inputs{
            if let Some(existing_player) = self.ecs.c.get_mut::<InputComp>(player_id as GlobalEntityID){
                existing_player.inputs.update_input_state(input_state);
            }
        }
        self.ecs.sim_systems(sim_meta);
    }
    fn render(&mut self, ctx: &mut Context, player_id: PlayerID, res: &RenderResourcesPtr){
        let timer = DT::start("RenderTime");
        super::bibble::render::render(&mut self.ecs, ctx, res, player_id as GlobalEntityID);
        if game::DEBUG_MSGS_ITS_LAGGING && rand::thread_rng().gen_bool(0.1){
            timer.stop();
        }
    }
    fn gen_render_resources(ctx: &mut Context) -> RenderResourcesPtr {
        let mut resources = GgEzRenderResources::load(ctx);

        return Arc::new(resources);
    }

    type Resources = GgEzRenderResources;
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
            map.register_type::<PlayerComp>();
            map.register_type::<RenderComp>();
            map.register_type::<SceneManager>();
            map.register_type::<ScenePersistent>();
            map.register_type::<LobbyManager>();
            map.register_type::<ClickableComp>();
            map.register_type::<UIComp>();
            map.register_type::<JigsawPieceComp>();
            map.register_type::<JigsawPlayerComp>();
            map.register_type::<JigsawMatComp>();
            map.register_type::<CursorComp>();
            map.register_type::<IgnoreHoverComp>();
            // map.register_type::<BenchStruct>();
            map
        },
        systems: vec![
            INPUT_PREPROC.clone(),
            BUTTON_SYS.clone(),
            RACE_BUTTON_SYS.clone(),
            MAP_BUTTON_SYS.clone(),
            PERFORMANCE_MAP.clone(),
            CAMERA_PAN_SYS.clone(),
            SEEKING_PROJECTILES_COMP.clone(),
            SELECTION_BOX_SYS.clone(),
            ABILITY_TARGETING.clone(),
            ABILITIES_SYS.clone(),
            VELOCITY_SYS.clone(),
            ORDERS_SYS.clone(),
            HIKER_SYS.clone(),
            HIKER_COLLISION_SYS.clone(),
            SHOOT_MOUSE_SYS.clone(),
            COLLISION_SYS.clone(),
            VELOCITY_WITH_INPUTS_SYS.clone(),
            WORKER_SYS.clone(),
            WEAPON_SYS.clone(),
            LIFE_SYS.clone(),
            LOSS_SYS.clone(),
            NO_LEAVE_MAP.clone(),
            LOBBY_SYS.clone(),
            JIGSAW_PIECE_SYS.clone(),
            JIGSAW_MAT_SYS.clone(),
            CURSOR_SYS.clone(),
            JIGSAW_PLAYER_SYS.clone(),
            UI_SYS.clone(),
            SCENE_SWITCHER_SYS.clone(),
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