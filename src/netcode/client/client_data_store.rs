use crate::netcode::common::sim_data::confirmed_data::ConfirmedData;
use crate::netcode::common::sim_data::superstore_seg::Superstore;
use crate::netcode::InputState;
use crate::netcode::common::sim_data::net_game_state::NetGameState;

pub struct ClientDataStore{
    pub confirmed_data: ConfirmedData,
    pub predicted_local: Superstore<InputState>,
}
impl ClientDataStore{
    pub fn new() -> Self{
        Self{
            confirmed_data: ConfirmedData::new(),
            predicted_local: Superstore::new(false),
        }
    }
    pub fn glean_connected_players(&mut self, state: &NetGameState){
        for connected_player in welcome_info.game_state.game_state{
            seg_data_storage.add_new_player(connected_player, self.welcome_info.game_state.get_simmed_frame_index() + 1);
        }
    }
}