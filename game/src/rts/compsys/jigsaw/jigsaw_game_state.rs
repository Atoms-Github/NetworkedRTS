use netcode::*;
use ggez::{*};
use std::sync::Arc;
use crate::ecs::{GlobalEntityID};
use ggez::graphics::{DrawParam, Text};
use nalgebra::Point2;
use crate::ecs::pending_entity::PendingEntity;
use serde_closure::internal::std::future::Pending;
pub use crate::utils::gett;
use crate::ecs::superb_ecs::{System, EcsConfig, SuperbEcs, EntStructureChanges};
use crate::rts::compsys::player::{PlayerComp};
use crate::rts::compsys::*;
use crate::bibble::data::data_types::{GameData, RaceID};
use crate::bibble::effect_resolver::revolver::Revolver;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::rts::game::render_resources::RenderResources;
use rand::Rng;
use netcode::common::net_game_state::GameState;
use crate::ecs::bblocky::comp_registration::{SuperbFunctions, FunctionMap};
use crate::ecs::comp_store::CompStorage;

pub const MAX_PLAYERS : usize = 16;
pub const SCENE_MAN_ENT_ID: GlobalEntityID = MAX_PLAYERS;

pub type UsingRenderResources = Arc<RenderResources>;

fn sys<C>(    run: fn(&mut CompStorage<C> /* Could add read only version here. */, &mut EntStructureChanges<C>, &SimMetadata),
           name: &'static str,) -> System<C>{
    System{
        run,
        name
    }


}
pub fn global_get_systems() -> Vec<System<GameStateJigsaw>>{
    vec![
        input_preproc_sys(),
        button_sys(),
        race_button_sys(),
        map_button_sys(),
        performance_map_sys(),
        camera_pan_sys(),
        seeking_projectiles_sys(),
        selection_box_sys(),
        ability_targeting_sys(),
        abilities_sys(),
        velocity_sys(),
        orders_sys(),
        hiker_sys(),
        hiker_collision_sys(),
        shoot_mouse_sys(),
        collision_sys(),
        velocity_with_inputs_sys(),
        worker_sys(),
        weapon_sys(),
        life_sys(),
        loss_sys(),
        leave_map_no_sys(),
        lobby_sys(),
        jigsaw_piece_sys(),
        jigsaw_mat_sys(),
        cursor_sys(),
        jigsaw_player_sys(),
        ui_sys(),
        scene_switcher_sys(),
    ]
}


#[derive(Clone, Serialize, Deserialize, Hash, Debug)]
pub struct GameStateJigsaw {
    ecs: SuperbEcs<Self>,
}

impl EcsConfig for GameStateJigsaw{
    fn get_systems<C>() -> Vec<System<C>> {
        todo!()
    }

    fn get_functions() -> &'static FunctionMap {
        return &crate::ecs::bblocky::comp_registration::FUNCTION_MAP;
        todo!()
    }
}



impl Default for GameStateJigsaw {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState for GameStateJigsaw {
    fn new() -> Self {
        Self{
            ecs: SuperbEcs::new(),
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
        super::super::bibble::render::render(&mut self.ecs, ctx, res, player_id as GlobalEntityID);
        if crate::DEBUG_MSGS_ITS_LAGGING && rand::thread_rng().gen_bool(0.1){
            timer.stop();
        }
    }
    fn gen_render_resources(ctx: &mut Context) -> RenderResourcesPtr {
        let mut resources = RenderResources::load(ctx);

        return Arc::new(resources);
    }

    type Resources = RenderResources;
}