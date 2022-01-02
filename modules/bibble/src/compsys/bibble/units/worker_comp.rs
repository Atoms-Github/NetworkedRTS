use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use game::pub_types::{PointFloat, PlayerID};
use bibble::::*;
use ggez::graphics::Rect;
use std::ops::Div;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct WorkerComp {
    pub resource_gain_per_ms: ResourceBlock,
}


pub static WORKER_SYS: System = System{
    run,
    name: "worker"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    for (worker_id, life, owned, worker) in
    CompIter3::<LifeComp, OwnedComp, WorkerComp>::new(c) {
        let resources_comp = c.get_mut::<OwnsResourcesComp>(owned.owner).unwrap();
        resources_comp.gain_block(&worker.resource_gain_per_ms, meta.delta);
    }

}