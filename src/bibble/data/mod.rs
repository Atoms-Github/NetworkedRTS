use data_types::GameData;

pub mod data_types;
mod races;


pub fn gen_game_data() -> GameData{
    let mut game_data = GameData{
        units: Default::default(),
        weapons: Default::default(),
        races: Default::default(),
        projectiles: Default::default(),
        abilities: Default::default()
    };

    races::gather_races(&mut game_data);


    return game_data;
}