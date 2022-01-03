use crate::*;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OwnedComp {
    pub owner: GlobalEntityID,
}