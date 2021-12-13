use std::collections::HashMap;
use crate::pub_types::{PlayerID, Shade};
use serde::{Serialize, Deserialize};

pub use crate::netcode::common::sim_data::input_state::InputState;
pub use crate::netcode::common::sim_data::input_state::ConnStatusChangeType;
use crate::netcode::netcode_types::ServerEvents;
use crate::netcode::common::input_state::InputState;

mod server;
mod client;
pub(crate) mod common;
mod netcode_types;

pub type PlayerInputs = HashMap<PlayerID, InputState>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InfoForSim {
    pub inputs_map: PlayerInputs,
    pub server_events: ServerEvents
}



pub fn server_main(hosting_ip: String){
    server::server_mode::server_main(hosting_ip);
}
pub fn client_main(player_name: String, connection_ip: String, preferred_id: i32){
    let lower_name = player_name.to_lowercase();
    let my_color : Shade = match &lower_name[..]{
        "atoms" => {
            Shade(0.258, 0.529, 0.960)
        }
        "quicktoast" => {
            Shade(0.156, 0.823, 0.372)
        }
        "oberdiah" => {
            Shade(0.882, 0.035, 0.101)
        }
        "legend" => {
            Shade(1.0, 1.0, 1.0) // White. ?
        }
        "numcake" => {
            Shade(0.956, 0.517, 0.121)
        }
        "shotekri" => {
            Shade(0.956, 0.145, 0.596)
        }
        "connorhs" => {
            Shade(0.917, 0.956, 0.145) // Yellow. ?
        }
        "lain" => {
            Shade(0.145, 0.956, 0.607) // Turquoise.
        }
        "guest2" => {
            Shade(0.537, 0.145, 0.956) // Purple.
        }
        "guest3" => {
            Shade(0.301, 0.301, 0.301) // Dark grey.
        }
        "guest4" => {
            Shade(0.588, 0.588, 0.588) // Light grey.
        }
        "guest5" => {
            Shade(0.588, 0.588, 0.588) // Beige.
        }
        _ => {
            Shade(0.0,0.0,0.0)
        }
    };
    client::client_mode::ClientApp::go(player_name, my_color, connection_ip, preferred_id);
}


