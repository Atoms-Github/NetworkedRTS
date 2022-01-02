use bibble::::data::data_types::GameData;

// mod robots;
mod sans_races;
mod robots_file;
mod quick_toasties;
// mod dragons;

pub fn gather_races(data: &mut GameData){
    robots_file::gather(data);
    // robots::gather(data);

    sans_races::gather(data);
    quick_toasties::gather(data);
    // dragons::gather(data);
}