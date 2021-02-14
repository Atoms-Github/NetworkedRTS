use std::collections::HashMap;
use serde::*;

use crate::gamecode::ecs::world::*;
use crate::gamecode::systems::render::*;
use crate::gamecode::systems::position::*;
use crate::gamecode::systems::velocity::*;
use crate::gamecode::systems::clickshooter::*;
use crate::gamecode::systems::wasdmover::*;
use crate::gamecode::systems::size::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::gamecode::systems::player::PlayerComp;
use crate::pub_types::{HashType, FrameIndex, PlayerID};
use crate::netcode::{InfoForSim, ConnStatusChangeType};
use ggez::Context;


#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct GameState{
    pub world: World,
    pub storages: Storages,
    /* Private */simmed_frame_index: FrameIndex,
}

impl Default for GameState{
    fn default() -> Self {
        Self::new()
    }
}
impl GameState{
    pub fn get_hash(&self) -> HashType{
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
    pub fn get_simmed_frame_index(&self) -> FrameIndex{
        return self.simmed_frame_index;
    }
    pub fn new() -> GameState{
        GameState{
            world: World::new(),
            storages: Storages::new(),
            simmed_frame_index: 0
        }
    }
    pub fn init_rts(&mut self){
        let mut pending = PendingEntities::new();

//        let mut pending_entity_online_player = PendingEntity::new();
//        pending_entity_online_player.add_component(PositionComp{ x: 0.0, y: 0.0 });
//        pending_entity_online_player.add_component(VelocityComp{ x: 0.0, y: 0.5 });
//        pending_entity_online_player.add_component(SizeComp{ x: 50.0, y: 50.0 });
//        pending_entity_online_player.add_component(RenderComp{ hue: (0,150,100)});
//        pending.create_entity(pending_entity_online_player);

        self.world.update_entities(&mut self.storages, pending);
    }
    pub fn init_new_player(&mut self, player_id: PlayerID){
        let mut pending = PendingEntities::new();

        let mut pending_player = PendingEntity::new();
        pending_player.add_component(PlayerComp{ player_id, connected: true } );
        pending.create_entity(pending_player);

        let mut pending_pawn = PendingEntity::new();
        pending_pawn.add_component(PositionComp{ x: 0.0, y: 0.0 });
        pending_pawn.add_component(VelocityComp{ x: 1.0, y: 0.0 });
        pending_pawn.add_component(SizeComp{ x: 50.0, y: 50.0 });
        pending_pawn.add_component(ClickShooterComp { owner_id: player_id, cooldown: 0.0 });
        pending_pawn.add_component(WasdMoverComp { owner_id: player_id });
        pending_pawn.add_component(RenderComp{ hue: (255, 150, 150)});
        pending.create_entity(pending_pawn);



        self.world.update_entities(&mut self.storages, pending);
    }
    pub fn simulate_tick(&mut self, sim_info: InfoForSim, delta: f32){
        for (player_id, input) in &sim_info.inputs_map{
            if input.conn_status_update == ConnStatusChangeType::Connecting{ // TODO: Temp. Will swap over for search for existing player object.
                log::trace!("StateInitingNewPlayer {}", *player_id);
                self.init_new_player(*player_id);
            }
            // TODO Disconnecting and player properties.
        }
        let mut pending = PendingEntities::new();

        secret_position_system(&self.world, &mut pending, &mut self.storages.position_s, &mut self.storages.velocity_s);
        secret_velocity_system(&self.world, &mut pending, &mut self.storages.position_s, &mut self.storages.velocity_s);
        secret_clickshooter_system(&self.world, &mut pending, &mut self.storages.velocity_s,
                                           &mut self.storages.click_shooter_s, &mut self.storages.position_s, &sim_info.inputs_map, self.simmed_frame_index);
        secret_wasdmover_system(&self.world, &mut pending, &mut self.storages.velocity_s,
                                   &mut self.storages.wasdmover_s, &sim_info.inputs_map, self.simmed_frame_index);

        self.world.update_entities(&mut self.storages, pending);

        self.simmed_frame_index += 1;
    }
    pub fn render(&mut self, ctx: &mut Context){
        secret_render_system(&self.world, &mut PendingEntities::new(),
                             &mut self.storages.position_s,
                             &mut self.storages.render_s,
                             &mut self.storages.size_s,
                             ctx);
    }
}