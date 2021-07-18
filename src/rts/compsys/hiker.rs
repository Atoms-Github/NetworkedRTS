
use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::MouseButton;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};

pub struct HikerComp { // Walks comp, but includes fliers and sailers too.
    pub destination: PointFloat, // TODO: Change to in_mem_vec.
    pub quest_importance: u8,
}


pub static HIKER_SYS: System<ResourcesPtr> = System{
    run
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){

    for (sel_box_id, sel_box, position, size, owned) in CompIter4::<SelBoxComp, PositionComp, SizeComp, OwnedComp>::new(c) {
        let mouse_pos = c.get::<InputComp>(owned.owner).unwrap().mouse_pos_game_world.clone();
        size.size = mouse_pos - position.pos;
    }


}


