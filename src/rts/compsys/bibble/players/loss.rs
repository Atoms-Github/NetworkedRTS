use ggez::event::MouseButton;

use crate::ecs::comp_store::CompStorage;
use crate::ecs::GlobalEntityID;
use crate::ecs::superb_ecs::{EntStructureChanges, System};
use crate::pub_types::{PointFloat, PlayerID};
use crate::rts::compsys::*;
use ggez::graphics::Rect;
use std::ops::Div;

use crate::bibble::data::data_types::{WeaponID, AbilityID, VirtualKeyCode, AbilityTargetType};
use crate::bibble::effect_resolver::revolver::Revolver;

pub static LOSS_SYS: System = System{
    run,
    name: "loss"
};
fn run(c: &mut CompStorage, ent_changes: &mut EntStructureChanges, meta: &SimMetadata){
    let scene = c.find_scene();

    if scene.current == SceneType::InGame{
        let mut alive_players = 0;
        for (player_id , input, player) in CompIter2::<InputComp, PlayerComp>::new(c) {
            if player.alive{
                if (input.inputs.primitive.is_keycode_pressed(VirtualKeyCode::F12)
                    && (input.inputs.primitive.is_keycode_pressed(VirtualKeyCode::LControl)
                    || input.inputs.primitive.is_keycode_pressed(VirtualKeyCode::RControl)))
                    || !player.connected /* Concede on disconnect. */{
                    // Delete everything owned by me.
                    for (entity, owned) in CompIter1::<OwnedComp>::new(c) {
                        if owned.owner == player_id{
                            ent_changes.deleted_entities.push(entity);
                        }
                    }
                }
                // Check for loss.
                let mut lost = true;
                for (entity, owned) in CompIter1::<OwnedComp>::new(c) {
                    if owned.owner == player_id{
                        lost = false;
                    }
                }
                if lost{
                    player.alive = false;
                }else{
                    alive_players += 1;
                }
            }
        }
        let min_alive_players = if scene.connected_players == 1{
            1
        }else{
            2
        };
        if alive_players < min_alive_players{
            scene.completed_rounds += 1;
            scene.next = SceneType::Lobby;
        }
    }

}










