use serde::{Serialize, Deserialize};
use crate::rts::GameState;
use crate::pub_types::{PlayerID, FrameIndex, HashType, ResourcesPtr};
use std::collections::{HashMap, BTreeMap};
use std::collections::hash_map::DefaultHasher;
use ggez::Context;
use crate::netcode::{InfoForSim, ConnStatusChangeType};
use std::hash::{Hash, Hasher};
use crate::netcode::common::sim_data::sim_data_storage::ServerEvent;
use crate::rts::game::game_state::Resources;
use std::sync::Arc;

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct NetPlayerProperty{
    pub waiting_on: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct NetGameState {
    pub game_state: GameState,
    players: BTreeMap<PlayerID, NetPlayerProperty>,
    simmed_frame_index: FrameIndex,
}

impl NetGameState {
    fn get_net_property_mut(&mut self, player_id: &PlayerID) -> &mut NetPlayerProperty{
        if !self.players.contains_key(player_id){
            self.players.insert(*player_id, NetPlayerProperty{
                waiting_on: false,
            });
        }
        return self.players.get_mut(&player_id).unwrap();
    }
    pub fn update_connected_players(&mut self, events: &Vec<ServerEvent>){
        for event in events{
            match event{
                ServerEvent::DisconnectPlayer(player_id) => {
                    let property = self.get_net_property_mut(player_id);
                    property.waiting_on = false;
                }
                ServerEvent::JoinPlayer(player_id, name) => {
                    let property = self.get_net_property_mut(player_id);
                    property.waiting_on = true;

                }
            }
        }
    }
    pub fn get_connected_players(&self) -> Vec<PlayerID>{
        let mut players = vec![];
        for (player_id, player_property) in self.players.iter(){
            if player_property.waiting_on{
                players.push(*player_id);
            }
        }
        return players;
    }
    pub fn get_disconnected_players(&self) -> Vec<PlayerID>{
        let mut players = vec![];
        for (player_id, player_property) in self.players.iter(){
            if !player_property.waiting_on{
                players.push(*player_id);
            }
        }
        return players;
    }
    pub fn get_hash(&self) -> HashType{
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
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
    pub fn simulate_tick(&mut self, sim_info: InfoForSim, res: &ResourcesPtr, delta: f32){
        for server_event in &sim_info.server_events{
            match server_event{
                ServerEvent::JoinPlayer(player_id, name) => {
                    assert!(sim_info.inputs_map.contains_key(player_id), "Player connected, but didn't have input state for that frame. Frame {}", self.get_simmed_frame_index() + 1);
                    self.game_state.player_connects(*player_id, (*name).clone());
                }
                ServerEvent::DisconnectPlayer(player_id) => {
                    self.game_state.player_disconnects(*player_id);
                }
            }
        }
        self.game_state.simulate_tick(sim_info.inputs_map, res, delta, self.simmed_frame_index);
        self.simmed_frame_index += 1;


    }
    pub fn render(&mut self, ctx: &mut Context){
        self.game_state.render(ctx)
    }
}