use crate::*;
use std::ops::Mul;
use nalgebra::{distance, distance_squared};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OrderInstance{
    pub ability: AbilityID,
    pub target: AbilityTargetInstance,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum AbilityTargetInstance{
    NO_TARGET,
    UNIT(GlobalEntityID),
    POINT(PointFloat),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum OrderState {
    NONE,
    MOVING,
    WAITING_FOR_COOLDOWN,
    CHANNELLING(f32),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OrdersComp {
    pub orders_queue: Vec<OrderInstance>,
    pub state: OrderState,
    pub executing_order_target_loc: PointFloat,
}
impl OrdersComp{
    pub fn enqueue(&mut self, order: OrderInstance, before: bool){
        if before{
            self.orders_queue.clear();
            self.orders_queue.push(order);
            // Abort current (potentially channelling) action, and move to new one.
            self.state = OrderState::MOVING;
        }else{
            self.orders_queue.push(order);
            if self.state == OrderState::NONE{
                self.state = OrderState::MOVING;
            }
        }
    }
    pub fn get_executing_order(&self) -> Option<&OrderInstance>{
        return self.orders_queue.get(0);
    }
    pub fn finish_order(&mut self) -> OrderInstance{
        let removed = self.orders_queue.remove(0);
        if self.orders_queue.len() > 0{
            self.state = OrderState::MOVING;
        }else{
            self.state = OrderState::NONE;
        }
        return removed;
    }
}

pub static ORDERS_SYS: System = System{
    run,
    name: "orders"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){

    // Check for dead target.
    for (unit_id, owned, orders, position, hiker)
    in CompIter4::<OwnedComp, OrdersComp, PositionComp, HikerComp>::new(c) {
        let mut dead = false;
        if let Some(executing_order) = orders.get_executing_order(){
            if let AbilityTargetInstance::UNIT(target_unit) = executing_order.target{
                if !c.ent_alive(target_unit){
                    dead = true;
                }
            }
        }
        if dead{
            orders.finish_order();
        }
    }
    // Set order target loc.
    for (unit_id, owned, orders, position, hiker)
    in CompIter4::<OwnedComp, OrdersComp, PositionComp, HikerComp>::new(c) {
        if let Some(current_order) = orders.get_executing_order(){
            orders.executing_order_target_loc = match &current_order.target{
                AbilityTargetInstance::NO_TARGET => {
                    position.pos.clone()
                }
                AbilityTargetInstance::UNIT(target_unit) => {
                    c.get1_unwrap::<PositionComp>(*target_unit).pos.clone()
                }
                AbilityTargetInstance::POINT(target) => {
                    target.clone()
                }
            } as PointFloat;
        }
    }
    // Check for in-range.
    for (unit_id, owned, orders, position, hiker)
    in CompIter4::<OwnedComp, OrdersComp, PositionComp, HikerComp>::new(c) {
        // These are the two states that care about range.
        if orders.state == OrderState::MOVING || orders.state == OrderState::WAITING_FOR_COOLDOWN{
            if let Some(current_order) = orders.get_executing_order(){
                let ability = unit_id.get_owner_tech_tree(c).get_ability(current_order.ability);
                let target_pos = &orders.executing_order_target_loc;
                let in_range = (position.pos.clone() - target_pos).magnitude_squared() <= ability.range.powf(2.0);
                if in_range{
                    // Start channelling/waiting.
                    orders.state = OrderState::WAITING_FOR_COOLDOWN;
                }else{
                    // Not in range. Move to it.
                    orders.state = OrderState::MOVING;
                }
            }
        }
    }
    // Check for ability cooled down.
    for (unit_id, owned, orders, position, abilities)
    in CompIter4::<OwnedComp, OrdersComp, PositionComp, AbilitiesComp>::new(c) {
        // These are the two states that care about range.
        if orders.state == OrderState::WAITING_FOR_COOLDOWN{
            if let Some(current_order) = orders.get_executing_order(){
                let ability = unit_id.get_owner_tech_tree(c).get_ability(current_order.ability);
                if abilities.get_ability(current_order.ability).time_since_use >= ability.cooldown{
                    orders.state = OrderState::CHANNELLING(0.0);
                }
            }
        }
    }
    // Move towards target.
    for (unit_id, owned, orders, position, hiker)
    in CompIter4::<OwnedComp, OrdersComp, PositionComp, HikerComp>::new(c) {
        if orders.state == OrderState::MOVING{
            let is_rare_frame = meta.meta.frame_index % 10 == 0;
            let already_has_destination = hiker.get_destination().is_some();
            let mut destination_matches = false;
            if let Some(existing_destination) = hiker.get_destination(){
                if existing_destination == orders.executing_order_target_loc{
                    destination_matches = true;
                }
            }
            // What we want:
            // If no destination set, or (10th frame and target moved),
            // then update destination. Pathfindng is expensive.
            if !already_has_destination || (is_rare_frame && !destination_matches){
                hiker.set_destination(orders.executing_order_target_loc.clone(), 128);
            }
        }
    }
    // Increment channelling timers.
    for (unit_id, orders)
    in CompIter1::<OrdersComp>::new(c) {
        if let OrderState::CHANNELLING(channel_time) = &mut orders.state{
            *channel_time += meta.meta.delta;
        }
    }

    let mut revolver = Revolver::new(c);
    // Finish orders.
    for (unit_id, orders, abilities, owned)
    in CompIter3::<OrdersComp, AbilitiesComp, OwnedComp>::new(c) {
        if let OrderState::CHANNELLING(channel_time) = orders.state.clone(){
            let tech_tree = unit_id.get_owner_tech_tree(c);
            let executing_order = orders.get_executing_order().unwrap();
            let ability = tech_tree.get_ability(executing_order.ability);
            if channel_time >= ability.casting_time{
                let executed_order = orders.finish_order();
                if c.get_mut_unwrap::<OwnsResourcesComp>(owned.owner).try_pay(ResourceType::BLUENESS, ability.cost){
                    abilities.get_ability_mut(executed_order.ability).time_since_use = 0.0;
                    // Now execute ability.
                    revolver.revolve_ability_execution(tech_tree, unit_id, executed_order.ability, executed_order.target);
                }

            }
        }
    }
    revolver.end().apply(c);
}












