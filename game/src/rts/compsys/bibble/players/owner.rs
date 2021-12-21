
use crate::ecs::GlobalEntityID;
use crate::rts::compsys::*;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OwnedComp {
    pub owner: GlobalEntityID,
}