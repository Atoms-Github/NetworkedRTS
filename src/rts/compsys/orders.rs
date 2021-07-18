
use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::MouseButton;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
use std::ops::Mul;

pub struct OrdersComp {
    // TODO: Have vec (mem-safe vec) of orders list. Will go down the orders list, doing them one by one, and redirecting them to the proper components.
}

pub static ORDERS_SYS: System<ResourcesPtr> = System{
    run
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    for (player_id, inputs) in CompIter1::<InputComp>::new(c) {
        if inputs.mode == InputMode::UnitsSelected && inputs.inputs.mouse_event == RtsMouseEvent::MouseDown(MouseButton::Right){
            for (unit_id, selectable, owned, hiker) in CompIter3::<SelectableComp, OwnedComp, HikerComp>::new(c) {
                if selectable.is_selected && owned.owner == player_id{
                    hiker.destination = Some(inputs.mouse_pos_game_world);
                }
            }
        }
    }
}


