use crate::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OwnsCommanderComp {
    pub ent_id: Option<GlobalEntityID>,
    pub selected_race: RaceID,
}



#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CommanderComp {
    pub alive: bool,
    pub race: RaceID,
}
