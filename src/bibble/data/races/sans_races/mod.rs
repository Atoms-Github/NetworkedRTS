use crate::bibble::data::data_types::{GameData, RaceID, RaceMould, EffectToPoint};

mod units;
mod weapons;
mod abilities;


pub fn gather(data: &mut GameData){
    weapons::weapons(data);
    units::units(data);
    abilities::abilities(data);
}