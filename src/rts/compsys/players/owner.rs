
use crate::ecs::GlobalEntityID;


pub struct OwnedComp {
    pub owner: GlobalEntityID,
}