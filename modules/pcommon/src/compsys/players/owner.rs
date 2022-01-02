
use crate::ecs::GlobalEntityID;
use bibble::::*;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OwnedComp {
    pub owner: GlobalEntityID,
}