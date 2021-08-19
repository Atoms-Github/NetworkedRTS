use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use crate::rts::game::game_state::{ARENA_ENT_ID, RenderResources};
use ggez::graphics::Rect;
use std::ops::Div;

use crate::bibble::data::data_types::{WeaponID, AbilityID, VirtualKeyCode, AbilityTargetType};
use crate::bibble::effect_resolver::revolver::Revolver;

pub static ABILITY_TARGETING: System<ResourcesPtr> = System{
    run,
    name: "ability_targeting"
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    // Check for starting ability targetting:
    for (player_id , input, resources_temp) in CompIter2::<InputComp, OwnsResourcesComp>::new(c) {
        if let RtsKeyEvent::KeyDown(down_key) = input.inputs.key_event{
            let data = player_id.get_player_tech_tree(c);
            'units :for (unit_id , owned, selectable, abilities, orders)
            in CompIter4::<OwnedComp, SelectableComp, AbilitiesComp, OrdersComp>::new(c) {
                if selectable.is_selected{
                    for ability_id in &abilities.abilities{
                        let ability_mould = data.get_ability(*ability_id);
                        if ability_mould.button_info.hotkey == down_key{
                            match ability_mould.targetting{
                                AbilityTargetType::NoTarget(_) => {
                                    orders.enqueue(OrderInstance{
                                        ability: *ability_id,
                                        target: AbilityTargetInstance::NO_TARGET
                                    }, !input.inputs.primitive.is_keycode_pressed(VirtualKeyCode::LShift));
                                }
                                AbilityTargetType::Unit(_) |
                                AbilityTargetType::Point(_)=> {
                                    input.start_targetting(*ability_id);
                                }
                            }

                            break 'units;
                        }
                    }
                }
            }
        }
    }
    // Check for stopping ability targeting.
    for (player_id , input, resources_temp) in CompIter2::<InputComp, OwnsResourcesComp>::new(c) {
        if let InputMode::TargettingAbility(using_ability_id) = input.mode.clone(){
            if let RtsMouseEvent::MouseDown(mouse_button) = input.inputs.mouse_event{
                match mouse_button{
                    MouseButton::Left => {
                        let data = player_id.get_player_tech_tree(c);
                        for (unit_id , owned, selectable, abilities, orders)
                        in CompIter4::<OwnedComp, SelectableComp, AbilitiesComp, OrdersComp>::new(c) {
                            if selectable.is_selected{
                                for ability_id in &abilities.abilities{
                                    if *ability_id == using_ability_id{
                                        orders.enqueue(OrderInstance{
                                            ability: using_ability_id,
                                            target: AbilityTargetInstance::POINT(input.mouse_pos_game_world.clone()),
                                        }, !input.inputs.primitive.is_keycode_pressed(VirtualKeyCode::LShift));
                                    }
                                }
                            }
                        }
                        input.mode = InputMode::None;
                    }
                    MouseButton::Right => {
                        input.mode = InputMode::None;
                    }
                    _ => {}
                }
            }
        }
    }
}










