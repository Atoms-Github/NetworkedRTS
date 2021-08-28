use crate::bibble::data::data_types::GameData;

mod robots;
mod sans_races;
mod quick_toasties;

pub fn gather_races(data: &mut GameData){
    robots::gather(data);
    sans_races::gather(data);
    quick_toasties::gather(data);
}