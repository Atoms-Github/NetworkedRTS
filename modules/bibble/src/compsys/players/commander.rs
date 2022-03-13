use crate::*;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CommanderComp {
    pub alive: bool,
    pub race: RaceID,
}
