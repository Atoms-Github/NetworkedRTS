use crate::*;
use becs::{CompStorage, System};
use netcode::common::net_game_state::StaticFrameData;
use netcode::SimQuality;
use crate::archetypes::new_cursor;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ScenePersistent{ // Means keep when scene changes.
    pub keep_alive: bool, // Need to have some sort of size.
}
impl ScenePersistent{
    pub fn delete_all_non_persistent(c: &CompStorage){
        for entity_id in c.query(vec![]){
            let persist = c.get::<ScenePersistent>(entity_id);
            if persist.is_none() || !persist.unwrap().keep_alive{
                c.req_delete_entity(entity_id);
            }
        }
    }
}