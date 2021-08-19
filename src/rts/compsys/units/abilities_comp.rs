use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use crate::rts::game::game_state::{ARENA_ENT_ID, RenderResources};
use ggez::graphics::Rect;
use std::ops::Div;

use crate::bibble::data::data_types::{WeaponID, AbilityID};
use crate::bibble::effect_resolver::revolver::Revolver;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AbilitiesComp {
    pub abilities: Vec<AbilityID>,
}


pub static ABILITIES_SYS: System<ResourcesPtr> = System{
    run,
    name: "abilities"
};
fn run(res: &ResourcesPtr, c: &mut CompStorage, ent_changes: &mut EntStructureChanges){
    // Check for starting ability targetting:
    for (player_id , input, resources_temp) in CompIter2::<InputComp, OwnsResourcesComp>::new(c) {
        if let RtsKeyEvent::KeyDown(down_key) = input.inputs.key_event{
            let data = player_id.get_player_tech_tree(c);
            'units :for (unit_id , owned, selectable, abilities)
            in CompIter3::<OwnedComp, SelectableComp, AbilitiesComp>::new(c) {
                if selectable.is_selected{
                    for ability_id in &abilities.abilities{
                        if data.get_ability(*ability_id).button_info.hotkey == down_key{
                            input.start_targetting(*ability_id);
                            break 'units;
                        }
                    }
                }
            }
        }
    }
}










