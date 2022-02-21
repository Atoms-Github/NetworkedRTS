use crate::bibble::data::data_types::{GameData, RaceID, RaceMould, EffectToPoint};

mod units;
mod race;
mod abilities;


pub fn gather(data: &mut GameData){
    units::units(data);
    race::race(data);
    abilities::abilities(data);
}