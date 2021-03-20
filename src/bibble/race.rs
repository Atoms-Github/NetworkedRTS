


pub struct GameData{
    pub races: Vec<Race>
}
impl GameData{
    pub fn new() -> Self{
        let mut game_data = GameData{
            races: vec![]
        };

        game_data.races.push(Race{
            units: vec![]
        });

        return game_data;
    }
}

pub struct Race{ // Pog.
    pub units: Vec<Unit>
}

pub struct Unit{ // Pog.

}