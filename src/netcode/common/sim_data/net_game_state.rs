use serde::{Serialize, Deserialize};
use crate::gamecode::GameState;
use crate::pub_types::{PlayerID, FrameIndex, HashType};
use std::collections::{HashMap, BTreeMap};
use std::collections::hash_map::DefaultHasher;
use ggez::Context;
use crate::netcode::{InfoForSim, ConnStatusChangeType};
use std::hash::{Hash, Hasher};
use crate::netcode::common::sim_data::sim_data_storage::ServerEvent;

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct NetPlayerProperty{
    waiting_on: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct NetGameState {
    pub game_state: GameState,
    pub players: BTreeMap<PlayerID, NetPlayerProperty>,
    simmed_frame_index: FrameIndex,
}

impl NetGameState {
    pub fn get_hash(&self) -> HashType{
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
    pub fn get_who_we_wait_for(&self) -> Vec<PlayerID>{
        let mut players = vec![];
        for (player, properties) in self.players.iter(){
            if properties.waiting_on {
                players.push(*player);
            }
        }
        return players;
    }
    pub fn get_simmed_frame_index(&self) -> FrameIndex{
        return self.simmed_frame_index;
    }
    pub fn new() -> Self {
        let mut net_state = Self {
            game_state: GameState::new(),
            simmed_frame_index: 0,
            players: Default::default()
        };
        net_state.game_state.init();
        return net_state;
    }
    pub fn simulate_tick(&mut self, sim_info: InfoForSim, delta: f32){
        for server_event in &sim_info.server_events{
            match server_event{
                ServerEvent::JoinPlayer(player_id) => {
                    self.game_state.player_connects(*player_id);
                }
                ServerEvent::DisconnectPlayer(player_id) => {
                    self.game_state.player_disconnects(*player_id);
                }
            }
        }

        self.game_state.simulate_tick(sim_info, delta, self.simmed_frame_index);
        self.simmed_frame_index += 1;
    }
    pub fn render(&mut self, ctx: &mut Context){
        self.game_state.render(ctx)
    }
}