use serde::{Serialize, Deserialize};
use crate::rts::GameState;
use crate::pub_types::{PlayerID, FrameIndex, HashType, RenderResourcesPtr, SimQuality, SimMetadata};
use std::collections::{HashMap, BTreeMap};
use std::collections::hash_map::DefaultHasher;
use ggez::Context;
use crate::netcode::{InfoForSim, ConnStatusChangeType};
use std::hash::{Hash, Hasher};
use crate::netcode::common::sim_data::sim_data_storage::ServerEvent;

use std::sync::Arc;
use std::fmt::Debug;
use serde::__private::Formatter;
use std::{fmt, fs, env};
use std::path::Path;
use std::fs::File;
use std::io::Write;
use zip::write::FileOptions;

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct NetPlayerProperty{
    pub waiting_on: bool,
}

#[derive(Clone, Serialize, Deserialize, Hash)]
pub struct NetGameState {
    pub game_state: GameState,
    players: BTreeMap<PlayerID, NetPlayerProperty>,
    simmed_frame_index: FrameIndex,
}
impl Debug for NetGameState{
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.debug_tuple("ItsAGameState")
             .finish()
        }
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
                ServerEvent::JoinPlayer(player_id, name, shade) => {
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
        let mut args_str: Vec<String> = env::args().collect();
        let mode = args_str[1].clone();
        let filename = format!("/states/{}/{}.zip",mode, self.simmed_frame_index);
        if !Path::new(filename.as_str()).exists(){
            let path = std::path::Path::new(filename.as_str());
            let prefix = path.parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();
            let data = format!("Hash: Frame {} state {:?}", self.simmed_frame_index, self.game_state);
            let mut file = File::create(filename.as_str()).expect("Unable to create file");
            // f.write_all(data.as_bytes()).expect("Unable to write data");
            let mut zip = zip::ZipWriter::new(file);

            zip.add_directory("test/", Default::default()).unwrap();

            let options = FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .unix_permissions(0o755);
            zip.start_file("test/e.txt", options).unwrap();
            zip.write_all(data.as_bytes()).unwrap();

            zip.finish().unwrap();

        }

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
    pub fn simulate_tick(&mut self, sim_info: InfoForSim, sim_meta: &SimMetadata){
        for server_event in &sim_info.server_events{
            match server_event{
                ServerEvent::JoinPlayer(player_id, name, shade) => {
                    assert!(sim_info.inputs_map.contains_key(player_id), "Player connected, but didn't have input state for that frame. Frame {}", self.get_simmed_frame_index() + 1);
                    self.game_state.player_connects(*player_id, name.clone(), shade.clone());
                }
                ServerEvent::DisconnectPlayer(player_id) => {
                    self.game_state.player_disconnects(*player_id);
                }
            }
        }
        self.game_state.simulate_tick(sim_info.inputs_map, sim_meta);
        self.simmed_frame_index += 1;


    }
    pub fn render(&mut self, ctx: &mut Context, player_id: PlayerID, res: &RenderResourcesPtr){
        self.game_state.render(ctx, player_id, res)
    }
}