use serde::{Serialize, Deserialize};
use crate::rts::GameState;
use crate::pub_types::{PlayerID, FrameIndex, HashType, RenderResourcesPtr, SimQuality, SimMetadata};
use std::collections::{HashMap, BTreeMap};
use std::collections::hash_map::DefaultHasher;
use ggez::Context;
use crate::netcode::{InfoForSim, ConnStatusChangeType, InputState};
use std::hash::{Hash, Hasher};
use crate::netcode::common::sim_data::confirmed_data::{ServerEvent, JoinType, ConfirmedData, SimDataQuery, SimDataOwner};

use std::sync::Arc;
use std::fmt::Debug;
use serde::__private::Formatter;
use std::{fmt, fs, env};
use std::path::Path;
use std::fs::File;
use std::io::Write;
use zip::write::FileOptions;
use crate::netcode::common::sim_data::superstore_seg::{Cap, Superstore};
use crate::ecs::bblocky::super_vec::SuperVec;

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct ConnectedPlayerProperty {
}

#[derive(Clone, Serialize, Deserialize, Hash)]
pub struct NetGameState {
    pub game_state: GameState,
    // players: BTreeMap<PlayerID, NetPlayerProperty>,
    connected_players: BTreeMap<PlayerID, ConnectedPlayerProperty>, // I.e Those who's connect events has already been simmed.
    simmed_frame_index: FrameIndex,
}
impl Debug for NetGameState{
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.debug_tuple("ItsAGameState")
             .finish()
        }
}

impl NetGameState {
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
            connected_players: Default::default(),
            simmed_frame_index: 0,
        };
        net_state.game_state.init();
        return net_state;
    }
    pub fn simulate(&mut self, sim_info: InfoForSim, sim_meta: &SimMetadata){
        self.game_state.simulate_tick(sim_info.inputs_map, sim_meta);
        self.simmed_frame_index += 1;

        // Update this at the end of the frame, because players have no inputs the frame they connect.
        for server_event in &sim_info.server_events{
            match server_event{
                ServerEvent::JoinPlayer(player_id, name, shade, reconnecting) => {
                    self.connected_players.insert(*player_id, ConnectedPlayerProperty{});
                    assert!(sim_info.inputs_map.contains_key(player_id), "Player connected, but didn't have input state for that frame. Frame {}", self.get_simmed_frame_index() + 1);
                    self.game_state.player_connects(*player_id, name.clone(), shade.clone());
                }
                ServerEvent::DisconnectPlayer(player_id) => {
                    self.game_state.player_disconnects(*player_id);
                    let existing = self.connected_players.remove(player_id);
                    assert!(existing.is_some());
                }
            }
        }
    }
    pub fn render(&mut self, ctx: &mut Context, player_id: PlayerID, res: &RenderResourcesPtr){
        self.game_state.render(ctx, player_id, res)
    }

    pub fn sim_far_as_pos(&mut self, data: &ConfirmedData) -> SimDataQuery{
        loop{
            let frame = self.simmed_frame_index + 1;
            let metadata = SimMetadata{
                delta: crate::netcode::common::time::timekeeping::FRAME_DURATION_MILLIS,
                quality: SimQuality::DETERMA,
                frame_index: frame
            };
            if let Some(events) = data.get_server_events(frame){
                let mut player_inputs: HashMap<PlayerID, InputState> = Default::default();

                for (player, properties) in self.connected_players {
                    if let Some(input_state) = data.get_input(frame, player){
                        player_inputs.insert(player, input_state.clone());
                    }else{
                        return SimDataQuery {
                            query_type: SimDataOwner::Player(player),
                            frame_offset: frame,
                        };
                    }
                }
                self.simulate(InfoForSim{
                    inputs_map: player_inputs,
                    server_events: events.clone()
                }, &metadata);
            }else{
                return SimDataQuery{
                    query_type : SimDataOwner::Server,
                    frame_offset : frame,
                };
            }
        }
    }
}