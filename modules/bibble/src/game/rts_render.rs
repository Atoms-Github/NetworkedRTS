use crate::*;
use ggez::{graphics, Context};
use ggez::graphics::{DrawParam, Text, Color, Mesh, MeshBuilder, Drawable, Rect};
use nalgebra::Point2;
use std::collections::BTreeMap;
use std::fmt;
use becs::superb_ecs::SuperbEcs;


pub fn simples_render(ecs: &mut SuperbEcs, ctx: &mut Context, res: &RenderResources, player_entity_id: GlobalEntityID){
    let mut cool_batcher = CoolBatcher::new();

    let player_camera = ecs.c.get::<CameraComp>(player_entity_id).unwrap();
    let player_input = ecs.c.get::<InputComp>(player_entity_id).unwrap();

    // Draw arena background.
    for (arena_id, arena_comp) in CompIter1::<ArenaComp>::new(&ecs.c){
        let screen_pos = player_camera.game_space_to_screen_space(arena_comp.get_centre());
        let screen_size = player_camera.game_size_to_screen_size(arena_comp.get_size());
        // e
        cool_batcher.add_rectangle(
            &screen_pos, &screen_size, graphics::Color::from_rgb(200,200,200), ZValue::Arena.g());
    }

    // Draw arena boxes.
    for (arena_id, arena_comp) in CompIter1::<ArenaComp>::new(&ecs.c){
        let base_pos_game = arena_comp.get_top_left();
        let small_size =  player_camera.game_size_to_screen_size(
            PointFloat::new(arena_comp.get_box_length() as f32 - 1.0, arena_comp.get_box_length() as f32 - 1.0)
        );
        for (grid_box, floor) in arena_comp.flooring.grid.iter_all(){
            let small_center_game = PointFloat::new((grid_box.x as f32 + 0.5) * arena_comp.get_box_length(),
                                                    (grid_box.y as f32 + 0.5) * arena_comp.get_box_length()) + &base_pos_game;
            let small_top_left_screen = player_camera.game_space_to_screen_space(small_center_game);
            let color = floor.get_color().to_color();
            cool_batcher.add_rectangle_rect(MyDrawParams{
                pos: small_top_left_screen,
                size: small_size.clone(),
            }, color, ZValue::ArenaBoxes.g());
        }
    }
    // Draw units.
    for (entity_id, position, render) in CompIter2::<PositionComp, RenderComp>::new(&ecs.c){
        let (screen_pos, on_screen_size) = player_camera.get_as_screen_transform(&ecs.c, entity_id);

        if let Some(life_comp) = ecs.c.get::<LifeComp>(entity_id){
            let bar_centre = screen_pos.clone() + PointFloat::new(0.0, -7.0);
            cool_batcher.add_progress_bar(&bar_centre, 5.0, life_comp.life,
                                          life_comp.max_life, Color::from_rgb(0,200,0),
                                          Color::from_rgb(255,0,0), ZValue::InGameUI.g());
        }
        if let Some(orders) = ecs.c.get::<OrdersComp>(entity_id){
            if let OrderState::CHANNELLING(channel_time) = &orders.state{
                let tech_tree = entity_id.get_owner_tech_tree(&ecs.c);
                let executing_order = orders.get_executing_order().unwrap();
                let ability = tech_tree.get_ability(executing_order.ability);

                let bar_centre = screen_pos.clone() + PointFloat::new(0.0, -14.0);

                let max_width = 100.0;
                let current_width = *channel_time / ability.casting_time * max_width;

                cool_batcher.add_progress_bar(&bar_centre, 5.0, current_width,
                                              max_width, Color::from_rgb(52, 210, 235),
                                              Color::from_rgb(0,0,0), ZValue::InGameUI.g());
            }
        }

        if let Some(owned_comp) = ecs.c.get::<OwnedComp>(entity_id){
            let mut border_width = 3.0;
            let mut border_color = ecs.c.get_unwrap::<PlayerComp>(owned_comp.owner).color.to_color();
            if let Some(selectable_comp) = ecs.c.get::<SelectableComp>(entity_id){
                if selectable_comp.is_selected{
                    border_width = 5.0;
                    // let push_amount = 0.25;
                    // let color_push = if border_color.r < 1.0-push_amount || border_color.g > 1.0-push_amount || border_color.b > 1.0-push_amount {
                    //     push_amount
                    // }else{
                    //     -push_amount
                    // };
                    // border_color = Color::new(border_color.r + color_push, border_color.g + color_push,
                    //                           border_color.b + color_push, border_color.a);
                }
            }
            let border_size = on_screen_size.clone() + PointFloat::new(border_width * 2.0, border_width * 2.0);
            cool_batcher.add_rectangle_rect(MyDrawParams{
                pos: screen_pos.clone(),
                size: border_size
            }, border_color, ZValue::InGameUIBelow.g());
        }
    }
    // // Draw names.
    // for (entity_id, owned, position) in CompIter2::<OwnedComp, PositionComp>::new(&ecs.c){
    //     let on_screen_pos = player_camera.game_space_to_screen_space(position.pos.clone());
    //     let player_name = ecs.c.get::<PlayerComp>(owned.owner).unwrap().name.clone();
    //     cool_batcher.add_text(on_screen_pos, player_name, graphics::Color::from((0,153,255)), 150);
    // }
    // Draw ded.
    let mut y = 100.0;
    for (entity_id, player) in CompIter1::<PlayerComp>::new(&ecs.c){
        if player.connected{
            y += 50.0;
            cool_batcher.add_text(PointFloat::new(20.0, y),
                                  player.name.clone(), Color::from_rgb(0,0,0), ZValue::UI.g());
            if player.alive{
                cool_batcher.add_text(PointFloat::new(150.0, y),
                                      "Alive".to_string(), Color::from_rgb(66, 245, 194), ZValue::UI.g());
            }else{
                cool_batcher.add_text(PointFloat::new(150.0, y),
                                      "Ded".to_string(), Color::from_rgb(230,0,0), ZValue::UI.g());
            }
        }
    }
    // Find ability buttons.
    let mut rendering_abilities = BTreeMap::new();
    for (unit_id, abilities, selectable, owned)
    in CompIter3::<AbilitiesComp, SelectableComp, OwnedComp>::new(&ecs.c){
        if owned.owner == player_entity_id && selectable.is_selected{
            for (i, ability_instance) in abilities.abilities.iter().enumerate(){
                let button_mould = &unit_id.get_owner_tech_tree(&ecs.c).get_ability(ability_instance.id).button_info;

                if rendering_abilities.get(&button_mould.hotkey).is_none(){
                    rendering_abilities.insert(button_mould.hotkey, ability_instance.id);
                }
            }
        }
    }
    // Draw ability buttons.
    for (i, (hotkey, ability_id)) in rendering_abilities.iter().enumerate(){
        let ability_mould = &player_entity_id.get_player_tech_tree(&ecs.c).get_ability(*ability_id);
        let screen_pos = PointFloat::new(50.0 + i as f32 * 100.0, 100.0);
        if let InputMode::TargettingAbility(targetting_ability_id) = player_input.mode{
            if targetting_ability_id == *ability_id{
                let my_params = MyDrawParams{
                    pos: PointFloat::new(screen_pos.x - 5.0, screen_pos.y - 5.0),
                    size: PointFloat::new(40.0,40.0),
                };
                cool_batcher.add_rectangle_rect(my_params,
                                                graphics::Color::from_rgb(255,204,0), ZValue::UI.g());
            }
        }
        cool_batcher.add_rectangle(&screen_pos, &PointFloat::new(30.0,30.0),
                                   Color::from(ability_mould.button_info.color), 192);
        cool_batcher.add_text(screen_pos, hotkey.my_to_string(), Color::from_rgb(0,0,0), 193);
        cool_batcher.add_text(screen_pos.clone() + PointFloat::new(0.0,-30.0),
                              ability_mould.cost.to_string(), Color::from_rgb(0,0,200), ZValue::AboveUI.g());

    }
    // Draw resources.
    for (player_id, owns_resources) in CompIter1::<OwnsResourcesComp>::new(&ecs.c){
        if player_id == player_entity_id{
            for res_index in 0..RESOURCES_COUNT{
                let on_screen_pos = PointFloat::new(50.0 + res_index as f32 * 100.0, 50.0);

                let res_count = owns_resources.get_counti(res_index).to_string();
                cool_batcher.add_text(on_screen_pos, res_count,
                                      graphics::Color::from((255, 255, 255)), ZValue::UI.g());
            }
        }
    }

    let mut test_param = DrawParam::new();
    // cool_batcher.add_image("factory.jpg".to_string(), test_param, 5);
    cool_batcher.gogo_draw(ctx, res);

}

