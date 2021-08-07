use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use crate::rts::game::game_state::{ARENA_ENT_ID, GameResources};
use ggez::graphics::Rect;
use std::ops::Div;


pub struct WorkerComp {
}


pub static WORKER_SYS: System<ResourcesPtr> = System{
    run,
    name: "worker"
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (worker_id, life, owned) in CompIter2::<LifeComp, OwnedComp>::new(c) {
        let resources_comp = c.get_mut::<OwnsResourcesComp>(owned.owner).unwrap();
        resources_comp.gain(ResourceType::BLUENESS, 1);

    }

}