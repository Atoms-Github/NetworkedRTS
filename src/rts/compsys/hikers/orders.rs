
use crate::rts::game::game_state::*;
use crate::rts::compsys::*;
use crate::pub_types::PointFloat;
use crate::ecs::superb_ecs::{System, EntStructureChanges};
use crate::ecs::comp_store::CompStorage;
use crate::ecs::pending_entity::PendingEntity;
use winit::MouseButton;
use winit::VirtualKeyCode;
use crate::ecs::ecs_macros::{CompIter3, CompIter4};
use std::ops::Mul;
use crate::bibble::data::data_types::AbilityID;
use mopa::Any;
use nalgebra::{distance, distance_squared};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OrderInstance{
    ability: AbilityID,
    target: AbilityTargetInstance,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum AbilityTargetInstance{
    NO_TARGET,
    UNIT(GlobalEntityID),
    POINT(PointFloat)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum OrderState {
    CHANNELLING(f32),
    MOVING,
    NONE,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OrdersComp {
    pub orders_queue: Vec<OrderInstance>,
    pub state: OrderState,
}
impl OrdersComp{
    pub fn enqueue(&mut self, order: OrderInstance, before: bool){
        if before{
            self.orders_queue.insert(0, order);
            // Abort current (potentially channelling) action, and move to new one.
            self.state = OrderState::MOVING;
        }else{
            self.orders_queue.push(order);
            if self.state == OrderState::NONE{
                self.state = OrderState::MOVING;
            }
        }


    }
}

pub static ORDERS_SYS: System<ResourcesPtr> = System{
    run,
    name: "orders"
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    // Check for move commands.
    for (player_id, inputs) in CompIter1::<InputComp>::new(c) {
        if inputs.mode == InputMode::UnitsSelected && inputs.inputs.mouse_event == RtsMouseEvent::MouseDown(MouseButton::Right){
            for (unit_id, selectable, owned, orders, hiker)
            in CompIter4::<SelectableComp, OwnedComp, OrdersComp, HikerComp>::new(c) {
                if selectable.is_selected && owned.owner == player_id{
                    let order = OrderInstance{
                        ability: AbilityID::WALK,
                        target: AbilityTargetInstance::POINT(inputs.mouse_pos_game_world.clone()),
                    };
                    orders.enqueue(order, !inputs.inputs.primitive.is_keycode_pressed(VirtualKeyCode::LShift));
                }
            }
        }
    }
    // Check for in-range to start channelling:
    for (unit_id, owned, orders, position, hiker)
    in CompIter4::<OwnedComp, OrdersComp, PositionComp, HikerComp>::new(c) {
        if orders.state == OrderState::MOVING{
            if let Some(current_order) = orders.orders_queue.get(0){
                let ability = unit_id.get_owner_tech_tree(c).get_ability(current_order.ability);
                let target_pos = match &current_order.target{
                    AbilityTargetInstance::NO_TARGET => {
                        position.pos.clone()
                    }
                    AbilityTargetInstance::UNIT(target_unit) => {
                        c.get1_unwrap::<PositionComp>(*target_unit).pos.clone()
                    }
                    AbilityTargetInstance::POINT(target) => {
                        target.clone()
                    }
                };
                // If in range, start channelling.
                if (position.pos.clone() - &target_pos).magnitude_squared() <= ability.range.powf(2.0){
                    // Start channelling.
                    orders.state = OrderState::CHANNELLING(0.0);
                }else{ // If out of range, set hiking destination.
                    hiker.destination = Some(target_pos);
                }
            }
        }
    }
    // Increment channelling timers.
    for (unit_id, orders)
    in CompIter1::<OrdersComp>::new(c) {
        if let OrderState::CHANNELLING(channel_time) = &mut orders.state{
            *channel_time += 16.66;
        }
    }

    // Finish orders.
    for (unit_id, orders)
    in CompIter1::<OrdersComp>::new(c) {
        if let OrderState::CHANNELLING(channel_time) = orders.state.clone(){
            if channel_time > 1.0{

            }
        }
    }

}












