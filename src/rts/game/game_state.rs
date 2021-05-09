use serde::*;
use crate::pub_types::{HashType, FrameIndex, PlayerID, ResourcesPtr, PointFloat};
use crate::netcode::{InfoForSim, PlayerInputs};
use ggez::{*};
use std::sync::Arc;
use crate::ecs::{ActiveEcs, GlobalEntityID};
use crate::ecs::System;
use crate::ecs::systems_man::SystemsMan;
use crate::rts::systems::velocity_sys::VeocitylSys;
use crate::ecs::my_anymap::SerdeAnyMap;
use crate::rts::comps::render_comp::RenderComp;
use crate::rts::comps::position_comp::PositionComp;
use ggez::graphics::DrawParam;
use crate::rts::comps::player_comp::PlayerComp;
use crate::rts::comps::owner_comp::OwnedComp;
use crate::rts::systems::velocity_with_inputs_sys::VelocityWithInputsSys;
use crate::rts::comps::velocity_with_inputs_comp::VelocityWithInputsComp;
use crate::rts::comps::velocity_component::VelocityComp;


const MAX_PLAYERS : usize = 4;


#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    ecs: ActiveEcs,
    systems_man: SystemsMan,
    player_count: usize
}

// No clone or serde.
pub struct Resources{
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}




impl GameState {
    pub fn new() -> Self {
        let mut systems = SystemsMan::new();
        systems.add_system(VeocitylSys {});
        systems.add_system(VelocityWithInputsSys {});
        Self{
            ecs: ActiveEcs::new(),
            systems_man: systems,
            player_count: 0
        }
    }
    pub fn init(&mut self){
        // Reserve entity ids 0 to 8ish so player ID and entity IDs match up.
        for player_index in 0..MAX_PLAYERS{
            assert_eq!(player_index, self.ecs.new_entity(SerdeAnyMap::new()))
        }
    }
    pub fn player_connects(&mut self, player_id: PlayerID, username: String){
        let mut components = SerdeAnyMap::new();
        components.insert(RenderComp{ colour: (255,255,255) });
        components.insert(PositionComp{ pos: PointFloat::new(1.0, 1.0) });
        components.insert(VelocityComp{ vel: PointFloat::new(0.0, 0.0) });
        components.insert( OwnedComp { owner: player_id as GlobalEntityID });
        components.insert( VelocityWithInputsComp{ speed: 2.0 });
        self.ecs.new_entity(components);


        if self.player_count < MAX_PLAYERS{
            let player_entity_id = player_id as GlobalEntityID;
            self.ecs.add_component(player_entity_id, PlayerComp{ inputs: Default::default()});

            self.player_count += 1;
        }

    }
    pub fn player_disconnects(&mut self, player_id: PlayerID){

    }
    pub fn simulate_tick(&mut self, inputs: PlayerInputs, res: &ResourcesPtr, delta: f32, frame_index: FrameIndex){
        for (player_id, input_state) in inputs{
            if let Some(existing_player) = self.ecs.get_mut::<PlayerComp>(player_id as GlobalEntityID){
                existing_player.inputs = input_state;
            }
        }
        self.ecs.run_systems(&self.systems_man);
    }
    pub fn render(&mut self, ctx: &mut Context){
        for entity in self.ecs.query(vec![crate::utils::crack_type_id::<RenderComp>(), crate::utils::crack_type_id::<PositionComp>()]){
            let position = self.ecs.get::<PositionComp>(entity).unwrap().clone();
            let render = self.ecs.get::<RenderComp>(entity).unwrap().clone();

            let params = DrawParam::new();

            let mode = graphics::DrawMode::fill();
            let bounds = graphics::Rect::new(position.pos.x, position.pos.y,50.0, 50.0);
            let color = graphics::Color::from(render.colour);

            let arena_background : graphics::Mesh = graphics::Mesh::new_rectangle(
                ctx,
                mode,
                bounds,
                color,
            ).unwrap();


            graphics::draw(ctx, &arena_background, params).unwrap();


            // let owner_id = e.my_wasdmover_comp(d).owner_id;
            //
            // let player_name = player_names.get(&owner_id).unwrap().clone();
            //
            // let fps = timer::fps(ctx);
            // let player_name_display = Text::new(player_name);
            //
            //
            // // When drawing through these calls, `DrawParam` will work as they are documented.
            // graphics::draw(
            //     ctx,
            //     &player_name_display,
            //     (Point2::new(e.my_position(d).x, e.my_position(d).y), graphics::WHITE),
            // ).unwrap();

        }
    }
    pub fn gen_resources() -> ResourcesPtr{
        let mut resources = Resources{

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