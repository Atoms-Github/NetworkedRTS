pub mod races;
pub use races::*;
pub use data_types::*;
use data_types::GameData;

pub mod data_types;

impl GameData{
    pub fn gen_game_data() -> Self{
        let mut game_data = GameData{
            units: Default::default(),
            races: Default::default(),
            abilities: Default::default()
        };
        races::gather_races(&mut game_data);


        return game_data;
    }
}
