use crate::rts::compsys::*;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::rts::game::game_state::GameResources;
use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;

pub struct InputComp{
    mode: InputMode,
}
pub enum InputMode{
    None,
    SelectionBox(GlobalEntityID),
    ClickUI(GlobalEntityID),
}

pub static INPUT_SYS: System<GameResources> = System{
    run
};
fn run(res: &GameResources, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (entity_id, velocity, position) in CompIter2::<VelocityComp, PositionComp>::new(c){
        position.pos += &velocity.vel;
    }
}