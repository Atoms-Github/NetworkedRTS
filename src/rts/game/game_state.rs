use serde::*;
use crate::pub_types::{HashType, FrameIndex, PlayerID, ResourcesPtr, PointFloat};
use crate::netcode::{InfoForSim, PlayerInputs};
use ggez::{*};
use std::sync::Arc;
use crate::ecs::{ActiveEcs, GlobalEntityID};
use crate::rts::comps::render_comp::RenderComp;
use crate::rts::comps::position_comp::PositionComp;
use ggez::graphics::{DrawParam, Text};
use crate::rts::comps::player_comp::PlayerComp;
use crate::rts::comps::owner_comp::OwnedComp;
use nalgebra::Point2;
use crate::rts::systems::velocity_sys::VelocityComp;
use crate::ecs::pending_entity::PendingEntity;
use crate::rts::systems::velocity_with_inputs_sys::VelocityWithInputsComp;
use serde_closure::internal::std::future::Pending;
pub use crate::utils::gett;
use crate::ecs::superb_ecs::System;


const MAX_PLAYERS : usize = 8;

pub type UsingResources = GameResources;
pub type UsingSystemsList = GameResources;

pub fn global_get_systems() -> Vec<System<UsingResources>>{
    vec![crate::rts::systems::velocity_sys::VELOCITY_SYS.clone(),
         crate::rts::systems::velocity_with_inputs_sys::VELOCITY_WITH_INPUTS_SYS.clone()]
}


#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    ecs: ActiveEcs<UsingResources>,
    player_count: usize
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
            let mut pending = PendingEntity::new();
            pending.add_comp(PlayerComp{ inputs: Default::default()});
            assert_eq!(player_index, self.ecs.c.create_entity(pending))
        }
    }
    pub fn player_connects(&mut self, player_id: PlayerID, username: String){
        let mut new_entity = PendingEntity::new();
        new_entity.add_comp(RenderComp{ colour: (255,255,255) });
        new_entity.add_comp(PositionComp{ pos: PointFloat::new(1.0, 1.0) });
        new_entity.add_comp(VelocityComp{ vel: PointFloat::new(0.0, 0.0) });
        new_entity.add_comp( OwnedComp { owner: player_id as GlobalEntityID });
        new_entity.add_comp( VelocityWithInputsComp{ speed: 2.0 });
        self.ecs.c.create_entity(new_entity);
    }
    pub fn player_disconnects(&mut self, player_id: PlayerID){

    }
    pub fn simulate_tick(&mut self, inputs: PlayerInputs, res: &ResourcesPtr, delta: f32, frame_index: FrameIndex){
        for (player_id, input_state) in inputs{
            if let Some(existing_player) = self.ecs.c.get_mut::<PlayerComp>(player_id as GlobalEntityID){
                existing_player.inputs = input_state;
            }
        }
        self.ecs.sim_systems(UsingResources{});
    }
    pub fn render(&mut self, ctx: &mut Context){
        for entity in self.ecs.c.query(vec![crate::utils::gett::<RenderComp>(), crate::utils::gett::<PositionComp>()]){
            let position = self.ecs.c.get::<PositionComp>(entity).unwrap().clone();
            let render = self.ecs.c.get::<RenderComp>(entity).unwrap().clone();

            let mode = graphics::DrawMode::fill();
            let bounds = graphics::Rect::new(position.pos.x, position.pos.y,50.0, 50.0);
            let color = graphics::Color::from(render.colour);

            let arena_background : graphics::Mesh = graphics::Mesh::new_rectangle(
                ctx,
                mode,
                bounds,
                color,
            ).unwrap();


            graphics::draw(
                ctx,
                &arena_background,
                DrawParam::new(),
            ).unwrap();
        }
        // for entity in self.ecs.query(vec![crate::utils::get_type_id::<OwnedComp>(), crate::utils::get_type_id::<PositionComp>()]){
        //     let position = self.ecs.get::<PositionComp>(entity).unwrap().clone();
        //     let owner = self.ecs.get::<OwnedComp>(entity).unwrap().owner;
        //     let player_name = self.ecs.get::<PlayerComp>(owner).unwrap().name.clone();
        //
        //     let player_name_display = Text::new(player_name);
        //
        //     graphics::draw(
        //         ctx,
        //         &player_name_display,
        //         (Point2::new(position.pos.x, position.pos.y), graphics::Color::from((0,153,255))),
        //     ).unwrap();
        // }
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