use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use ggez::graphics::Rect;
use std::ops::Div;

use crate::bibble::data::data_types::{WeaponID, AbilityID, VirtualKeyCode};
use crate::bibble::effect_resolver::revolver::Revolver;
use crate::pub_types::common::timekeeping::SpeedTimer;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct WeaponComp {
    pub time_since_shot: f32,
    pub wep_ability_id: AbilityID,
}

pub fn weapon_sys<C>() -> System<C>{
    System{
        run,
        name: "weapon"
    }
}
fn run<C>(c: &mut CompStorage<C>, ent_changes: &mut EntStructureChanges<C>, meta: &SimMetadata){

    // Check for move commands.
    for (player_id, inputs) in CompIter1::<InputComp>::new(c) {
        if inputs.mode == InputMode::None && inputs.inputs.mouse_event == RtsMouseEvent::MouseDown(MouseButton::Right){
            for (unit_id, selectable, owned, orders, hiker, abilities)
            in CompIter5::<SelectableComp, OwnedComp, OrdersComp, HikerComp, AbilitiesComp>::new(c) {
                if selectable.is_selected && owned.owner == player_id{
                    let order = OrderInstance{
                        ability: {
                            if inputs.inputs.primitive.is_keycode_pressed(VirtualKeyCode::LControl){
                                AbilityID::WALK
                            }else{
                                AbilityID::ATTACK_GROUND
                            }
                        },
                        target: AbilityTargetInstance::POINT(inputs.mouse_pos_game_world.clone()),
                    };
                    // Don't give structures the move command.
                    if abilities.has_ability(order.ability){
                        orders.enqueue(order, !inputs.inputs.primitive.is_keycode_pressed(VirtualKeyCode::LShift));
                    }
                }
            }
        }
    }

    let mut revolver = Revolver::new(c);
    // Increment time since shot.
    for (shooter_id, weapon, owned_shooter, position_shooter, orders)
    in CompIter4::<WeaponComp, OwnedComp, PositionComp, OrdersComp>::new(c) {
        weapon.time_since_shot += meta.delta;
    }
    if let Some(arena) = c.find_arena(){
        let mut timer = SpeedTimer::new();
        // Check for queuing up 'shoot once' commands.
        for (shooter_id, weapon, owned_shooter, position_shooter, orders)
        in CompIter4::<WeaponComp, OwnedComp, PositionComp, OrdersComp>::new(c) {
            // timer.start();
            let executing_order = orders.get_executing_order();
            if executing_order.is_none() || executing_order.unwrap().ability == AbilityID::ATTACK_GROUND{
                let data = shooter_id.get_owner_tech_tree(c);
                let ability_mould = data.get_ability(weapon.wep_ability_id);
                let nearby_units = arena.get_nearby_performance_map_entities(&position_shooter.pos, ability_mould.range);
                // Don't check cooldowns here. That's done on ability execution.
                // timer.end();
                for target_id in nearby_units {
                    let owned_target_owner = c.get_mut_unwrap::<OwnedComp>(*target_id).owner;
                    // let owned_target_owner = 0;
                    // let position_target= PointFloat::new(3.0,1.0);
                    let position_target= c.get_mut_unwrap::<PositionComp>(*target_id).pos.clone();
                    let in_range = (position_target - &position_shooter.pos).magnitude() < ability_mould.range;
                    if owned_target_owner != owned_shooter.owner && in_range{
                        orders.enqueue(OrderInstance{
                            ability: weapon.wep_ability_id,
                            target: AbilityTargetInstance::UNIT(*target_id)
                        }, true);
                        break;
                    }
                }

            }
        }

    }


    revolver.end().move_into(ent_changes);
}