use crate::bibble::data::data_types::GameData;

mod robots;

pub fn gather_races(data: &mut GameData){
    robots::gather(data);
}