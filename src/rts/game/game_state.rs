use serde::*;
use crate::pub_types::{HashType, FrameIndex, PlayerID, ResourcesPtr, PointFloat};
use crate::netcode::{InfoForSim, PlayerInputs};
use ggez::{*};
use std::sync::Arc;
use crate::ecs::{ActiveEcs, GlobalEntityID};
use ggez::graphics::{DrawParam, Text};
use nalgebra::Point2;
use crate::ecs::pending_entity::PendingEntity;
use serde_closure::internal::std::future::Pending;
pub use crate::utils::gett;
use crate::ecs::superb_ecs::System;
use crate::rts::compsys::player::{PlayerComp, PLAYER_NAME_SIZE_MAX};
use crate::rts::compsys::*;


const MAX_PLAYERS : usize = 10;
pub const ARENA_ENT_ID: GlobalEntityID = MAX_PLAYERS;

pub type UsingResources = Arc<GameResources>;

pub fn global_get_systems() -> Vec<System<UsingResources>>{
    vec![
        INPUT_PREPROC.clone(),
        CAMERA_PAN_SYS.clone(),
        VELOCITY_SYS.clone(),
        INPUT_SELECTION_BOX.clone(),
        HIKER_SYS.clone(),
        SHOOT_MOUSE_SYS.clone(),
        COLLISION_SYS.clone(),
        VELOCITY_WITH_INPUTS_SYS.clone()
    ]
}


#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    ecs: ActiveEcs<UsingResources>,
    player_count: usize,
}

// No clone or serde.
pub struct GameResources {
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self{
            ecs: ActiveEcs::new(global_get_systems()),
            player_count: 0,
        }
    }
    pub fn init(&mut self){
        //Reserve entity ids 0 to 8ish so player ID and entity IDs match up.
        for player_index in 0..MAX_PLAYERS{
            let mut pending = PendingEntity::new_player(player_index as GlobalEntityID);
            assert_eq!(player_index, self.ecs.c.create_entity(pending))
        }
        assert_eq!(self.ecs.c.create_entity(PendingEntity::new_arena()), ARENA_ENT_ID);
    }
    pub fn player_connects(&mut self, player_id: PlayerID, username: String){

        let new_entity = PendingEntity::new_test_unit(player_id as GlobalEntityID, PointFloat::new(0.0, 0.0));
        self.ecs.c.create_entity(new_entity);

        self.ecs.c.get_mut::<PlayerComp>(player_id as GlobalEntityID).unwrap().name = crate::utils::pad_name(username);
    }
    pub fn player_disconnects(&mut self, player_id: PlayerID){

    }
    pub fn simulate_tick(&mut self, inputs: PlayerInputs, res: &ResourcesPtr, delta: f32, frame_index: FrameIndex){
        for (player_id, input_state) in inputs{
            if let Some(existing_player) = self.ecs.c.get_mut::<InputComp>(player_id as GlobalEntityID){

                existing_player.inputs.set_input_state(input_state);
            }
        }
        self.ecs.sim_systems(res);
    }
    pub fn render(&mut self, ctx: &mut Context, player_id: PlayerID){
        crate::rts::compsys::render::render(&mut self.ecs, ctx, player_id as GlobalEntityID);
    }
    pub fn gen_resources() -> ResourcesPtr{
        let mut resources = GameResources {
        };
        return Arc::new(resources);
    }
}






// #[derive(Clone, Serialize, Deserialize, Debug, Hash)]
// pub struct GameState {
//     pub world: World,
//     pub storages: Storages,
//     pub player_names: BTreeMap<PlayerID, String>,
//
// }
// impl GameState {
//     pub fn new() -> GameState {
//         GameState {
//             world: World::new(),
//             storages: Storages::new(),
//             player_names: Default::default()
//         }
//     }
//     pub fn init(&mut self){
//         let mut pending = PendingEntities::new();
//
//        let mut pending_entity_online_player = PendingEntity::new();
//        pending_entity_online_player.add_component(PositionComp{ x: 0.0, y: 0.0 });
//        pending_entity_online_player.add_component(VelocityComp{ x: 0.0, y: 0.5 });
//        pending_entity_online_player.add_component(SizeComp{ x: 50.0, y: 50.0 });
//        pending_entity_online_player.add_component(RenderComp{ hue: (0,150,100)});
//        pending.create_entity(pending_entity_online_player);
//
//         self.world.update_entities(&mut self.storages, pending);
//     }
//     pub fn player_connects(&mut self, player_id: PlayerID, username: String){
//         let mut pending = PendingEntities::new();
//
//         let mut pending_player = PendingEntity::new();
//         pending_player.add_component(PlayerComp{ player_id, connected: true } );
//         pending.create_entity(pending_player);
//
//         let mut pending_pawn = PendingEntity::new();
//         pending_pawn.add_component(PositionComp{ x: 0.0, y: 0.0 });
//         pending_pawn.add_component(VelocityComp{ x: 1.0, y: 0.0 });
//         pending_pawn.add_component(SizeComp{ x: 50.0, y: 50.0 });
//         pending_pawn.add_component(ClickShooterComp { owner_id: player_id, cooldown: 0.0 });
//         pending_pawn.add_component(WasdMoverComp { owner_id: player_id });
//         pending_pawn.add_component(RenderComp{ hue: (255, 150, 150)});
//         pending.create_entity(pending_pawn);
//
//         self.player_names.insert(player_id, username);
//
//         self.world.update_entities(&mut self.storages, pending);
//     }
//     pub fn player_disconnects(&mut self, player_id: PlayerID){
//
//     }
//     pub fn simulate_tick(&mut self, inputs: PlayerInputs, delta: f32, frame_index: FrameIndex){
//         let mut pending = PendingEntities::new();
//
//         secret_position_system(&self.world, &mut pending, &mut self.storages.position_s, &mut self.storages.velocity_s);
//         secret_velocity_system(&self.world, &mut pending, &mut self.storages.position_s, &mut self.storages.velocity_s);
//         secret_clickshooter_system(&self.world, &mut pending, &mut self.storages.velocity_s,
//                                            &mut self.storages.click_shooter_s, &mut self.storages.position_s, &inputs, frame_index);
//         secret_wasdmover_system(&self.world, &mut pending, &mut self.storages.velocity_s,
//                                    &mut self.storages.wasdmover_s, &inputs, frame_index);
//
//         self.world.update_entities(&mut self.storages, pending);
//     }
//     pub fn render(&mut self, ctx: &mut Context){
//         secret_render_system(&self.world, &mut PendingEntities::new(),
//                              &mut self.storages.position_s,
//                              &mut self.storages.render_s,
//                              &mut self.storages.size_s,
//                              &mut self.storages.wasdmover_s,
//                              &self.player_names,
//                              ctx);
//     }
// }