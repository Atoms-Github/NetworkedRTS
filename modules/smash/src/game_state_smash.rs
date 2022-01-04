use crate::*;
use netcode::common::net_game_state::GameState;
use std::sync::Arc;


#[derive(Serialize, Deserialize, Clone, Hash, Debug)]
pub struct GameStateSmash{
    ecs: SuperbEcs
}
impl GameState for GameStateSmash{
    fn new() -> Self {
        Self{
            ecs: SuperbEcs::new(get_config())
        }
    }

    fn init(&mut self) {
        todo!()
    }

    fn player_connects(&mut self, player_id: PlayerID, username: String, color: Shade) {
        todo!()
    }

    fn player_disconnects(&mut self, player_id: PlayerID) {
        todo!()
    }

    fn simulate_tick(&mut self, inputs: PlayerInputs, sim_meta: &SimMetadata) {
        todo!()
    }

    fn render(&mut self, ctx: &mut ggez::context::Context, player_id: PlayerID, res: &Arc<Self::Resources>) {
        todo!()
    }

    fn gen_render_resources(ctx: &mut ggez::context::Context) -> Arc<Self::Resources> {
        todo!()
    }

    type Resources = RenderResources;
}

fn get_config() -> EcsConfig{
    EcsConfig{
        functions: {
            let mut map = FunctionMap::default();
            map.register_type::<ShootMouseComp>();
            map.register_type::<VelocityComp>();
            map.register_type::<VelocityWithInputsComp>();
            map.register_type::<PositionComp>();
            map.register_type::<RadiusComp>();
            map.register_type::<SizeComp>();
            map.register_type::<CollisionComp>();
            map.register_type::<HikerComp>();
            map.register_type::<HikerCollisionComp>();
            map.register_type::<LifeComp>();
            map.register_type::<OrdersComp>();
            map.register_type::<SelectableComp>();
            map.register_type::<CameraComp>();
            map.register_type::<InputComp>();
            map.register_type::<SelectableComp>();
            map.register_type::<SelBoxComp>();
            map.register_type::<OwnedComp>();
            map.register_type::<OwnsResourcesComp>();
            map.register_type::<PlayerComp>();
            map.register_type::<ArenaComp>();
            map.register_type::<AbilitiesComp>();
            map.register_type::<WeaponComp>();
            map.register_type::<WorkerComp>();
            map.register_type::<RenderComp>();
            map.register_type::<TechTreeComp>();
            map.register_type::<SeekingProjComp>();
            map.register_type::<SceneManager>();
            map.register_type::<ScenePersistent>();
            map.register_type::<LobbyManager>();
            map.register_type::<ClickableComp>();
            map.register_type::<RaceButtonComp>();
            map.register_type::<MapButtonComp>();
            map.register_type::<UIComp>();
            map.register_type::<UnitStructureComp>();
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