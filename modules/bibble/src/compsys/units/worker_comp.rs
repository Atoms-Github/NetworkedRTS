use crate::*;
use ggez::event::MouseButton;

use ggez::graphics::Rect;
use std::ops::Div;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct WorkerComp {
    pub resource_gain_per_ms: ResourceBlock<CommanderProperty>,
}


pub static WORKER_SYS: System = System{
    run,
    name: "worker"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    for (worker_id, life, owned, worker) in
    CompIter3::<LifeComp, OwnedComp, WorkerComp>::new(c) {
        let resources_comp = c.get_mut::<OwnsResourcesComp<CommanderProperty>>(owned.owner).unwrap();
        resources_comp.gain_block(&worker.resource_gain_per_ms, meta.meta.delta);
    }

}
