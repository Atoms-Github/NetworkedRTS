use ggez::event::{KeyCode, MouseButton};
use nalgebra::Point2;

use netcode::common::net_game_state::StaticFrameData;

use crate::*;
use crate::utils::gett;

pub static INPUT_PREPROC: System = System{
    run,
    name: "input_preproc"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    let entity_list = c.query_sorted(vec![gett::<PositionComp>(), gett::<SizeComp>(), gett::<RenderComp>()],
    |comp_store, entity| {comp_store.get_unwrap::<RenderComp>(entity).z});

    for (player_id, camera, input, player) in CompIter3::<CameraComp, InputComp, PlayerComp>::new(c){
        if !player.connected{
            continue;
        }
        input.inputs.update_input_state(meta.sim_info.inputs_map.get(&(player_id as PlayerID)).unwrap().clone());
        input.mouse_pos_game_world = camera.screen_space_to_game_space(input.inputs.primitive.get_mouse_loc().clone());
        input.hovered_entity = None;
        for ent_id in &entity_list{
            if c.get::<IgnoreHoverComp>(*ent_id).is_none(){
                let (screenpos, screensize) = camera.get_as_screen_transform(c, *ent_id);
                let screen_rect = screenpos.to_ggez_rect(&screensize);
                if screen_rect.contains(input.inputs.primitive.get_mouse_loc().to_point()){
                    input.hovered_entity = Some(*ent_id);
                }
            }
        }
    }
}

