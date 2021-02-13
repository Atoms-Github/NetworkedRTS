use std::collections::HashMap;
use crate::pub_types::PlayerID;
use crate::netcode::common::sim_data::input_state::InputState;

mod server;
mod client;
mod common;
mod netcode_types;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InfoForSim {
    pub inputs_map: HashMap<PlayerID, InputState>
}


pub trait GameState{
    fn simulate_tick(&mut self, sim_info: InfoForSim, delta: f32);
    fn init_new_player(&mut self, player_id: PlayerID);
    fn new_init() -> Self;
}

pub fn server_main(hosting_ip: String){
    server::server_mode::server_main(hosting_ip);
}
pub fn client_main(player_name: String, connection_ip: String, preferred_id: i32){
    client::client_mode::ClientApp::go(player_name, connection_ip, preferred_id);
}


