use crate::*;
use netcode::common::net_game_state::GameState;
use std::sync::Arc;



#[derive(Serialize, Deserialize, Clone, Hash, Debug)]
pub struct GameStateSmash{
    #[serde(deserialize_with = "state_deserialize")]
    ecs: SuperbEcs
}
impl GameState for GameStateSmash{
    fn new() -> Self {
        Self{
            ecs: SuperbEcs::new(get_config())
        }
    }

    fn init(&mut self){
        //Reserve entity ids 0 to 8ish so player ID and entity IDs match up.
        for player_index in 0..8{
            let mut pending = archetypes::new_player(player_index as GlobalEntityID);
            self.ecs.c.req_create_entity(pending);
        }
        self.ecs.c.req_create_entity(crate::archetypes::new_arena());
    }
    fn player_connects(&mut self, player_id: PlayerID, username: String, color: Shade) {
        let player_ent_id = player_id as GlobalEntityID;
        let pawn = crate::archetypes::new_wasd_pawn(player_ent_id, PointFloat::new(0.0,0.0), color);
        self.ecs.c.req_create_entity(pawn);
        let cursor = pcommon::archetypes::new_cursor(player_ent_id, color, 100);
        self.ecs.c.req_create_entity(cursor);


        self.ecs.c.get_mut::<PlayerComp>(player_ent_id).unwrap().name = username;
        self.ecs.c.get_mut::<PlayerComp>(player_ent_id).unwrap().color = color;
        self.ecs.c.get_mut::<PlayerComp>(player_ent_id).unwrap().connected = true;
    }

    fn player_disconnects(&mut self, player_id: PlayerID) {

    }

    fn simulate_tick(&mut self, inputs: PlayerInputs, sim_meta: &SimMetadata) {
        let data = StaticFrameData{
            meta: sim_meta,
            inputs: &inputs
        };
        self.ecs.sim_systems(&data);
    }

    fn render(&mut self, ctx: &mut ggez::Context, player_id: PlayerID, res: &Arc<Self::Resources>) {
        pcommon::simples_render(&mut self.ecs, ctx, res, player_id as GlobalEntityID);
    }

    fn gen_render_resources(ctx: &mut ggez::Context) -> Arc<Self::Resources> {
        let mut resources = RenderResources::load(ctx);

        return Arc::new(resources);
    }

    type Resources = RenderResources;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq, Hash, Copy)]
pub enum SmashPlayerProperty {
    Score,
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
            map.register_type::<LifeComp>();
            map.register_type::<CameraComp>();
            map.register_type::<InputComp>();
            map.register_type::<OwnedComp>();
            map.register_type::<OwnsResourcesComp<SmashPlayerProperty>>();
            map.register_type::<PlayerComp>();
            map.register_type::<RenderComp>();
            map.register_type::<ClickableComp>();
            map.register_type::<UIComp>();
            map.register_type::<CursorComp>();
            map.register_type::<IgnoreHoverComp>();
            map
        },
        systems: vec![
            INPUT_PREPROC.clone(),
            BUTTON_SYS.clone(),
            PERFORMANCE_MAP.clone(),
            CAMERA_PAN_SYS.clone(),
            VELOCITY_SYS.clone(),
            SHOOT_MOUSE_SYS.clone(),
            COLLISION_SYS.clone(),
            VELOCITY_WITH_INPUTS_SYS.clone(),
            LIFE_SYS.clone(),
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