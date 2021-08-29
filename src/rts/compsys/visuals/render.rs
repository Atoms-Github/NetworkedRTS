use ggez::{graphics, Context};
use ggez::graphics::{DrawParam, Text, Color, Mesh, MeshBuilder, Drawable};
use crate::utils::gett;
use crate::rts::compsys::*;
use crate::ecs::{ActiveEcs, GlobalEntityID};
use crate::rts::game::game_state::UsingRenderResources;
use nalgebra::Point2;
use crate::rts::compsys::owns_resources::{OwnsResourcesComp, RESOURCES_COUNT, ResourceType};
use crate::bibble::data::data_types::AbilityID;
use std::collections::BTreeMap;
use winit::VirtualKeyCode;
use std::fmt;
use crate::netcode::common::time::timekeeping::DT;
use rand::Rng;
use crate::rts::game::cool_batcher::CoolBatcher;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RenderComp{
    pub colour: (u8, u8, u8)
}

pub fn render(ecs: &mut ActiveEcs, ctx: &mut Context, res: &RenderResourcesPtr, player_entity_id: GlobalEntityID){
    let timer = DT::start("Render");

    let mut cool_batcher = CoolBatcher::new();

    let player_camera = ecs.c.get::<CameraComp>(player_entity_id).unwrap();
    let player_input = ecs.c.get::<InputComp>(player_entity_id).unwrap();



    // Draw arena background.
    for (arena_id, arena_comp) in CompIter1::<ArenaComp>::new(&ecs.c){
        let screen_pos = player_camera.game_space_to_screen_space(arena_comp.get_top_left());
        let screen_size = player_camera.game_size_to_screen_size(arena_comp.get_size());
        cool_batcher.add_rectangle(&screen_pos, &screen_size, graphics::Color::from_rgb(200,200,200), 20);
    }

    // Draw arena boxes.
    for (arena_id, arena_comp) in CompIter1::<ArenaComp>::new(&ecs.c){
        let base_pos_game = arena_comp.get_top_left();
        let small_size =  player_camera.game_size_to_screen_size(
            PointFloat::new(arena_comp.get_box_length() as f32 - 1.0, arena_comp.get_box_length() as f32 - 1.0)
        );
        for x in 0..arena_comp.pathing.len(){
            for y in 0..arena_comp.pathing[x].len(){
                let small_top_left_game = PointFloat::new((x * arena_comp.get_box_length()) as f32,
                                                     (y * arena_comp.get_box_length()) as f32) + &base_pos_game;
                let small_top_left_screen = player_camera.game_space_to_screen_space(small_top_left_game);
                let color = {
                    if arena_comp.pathing[x][y]{
                        graphics::Color::from_rgba(100,180,100, 100)
                    }else{
                        graphics::Color::from_rgba(180,100,100, 100)
                    }
                };
                let rect = graphics::Rect::new(small_top_left_screen.x, small_top_left_screen.y, small_size.x, small_size.y);
                cool_batcher.add_rectangle_rect(rect, color, 50);
            }
        }
    }

    // Draw units.
    for (entity_id, position, render) in CompIter2::<PositionComp, RenderComp>::new(&ecs.c){
        let (on_screen_pos, on_screen_size) = player_camera.get_as_screen_coords(&ecs.c, entity_id);
        cool_batcher.add_rectangle(&on_screen_pos, &on_screen_size,
                                   graphics::Color::from(render.colour), 128);
        if let Some(life_comp) = ecs.c.get::<LifeComp>(entity_id){
            cool_batcher.add_rectangle(&on_screen_pos, &PointFloat::new(life_comp.max_life, 5.0),
                                       graphics::Color::from_rgb(200,0,0), 150);
            cool_batcher.add_rectangle(&on_screen_pos, &PointFloat::new(life_comp.life, 5.0),
                                       graphics::Color::from_rgb(0,200,0), 160);
        }
        if let Some(selectable_comp) = ecs.c.get::<SelectableComp>(entity_id){
            if selectable_comp.is_selected{
                cool_batcher.add_rectangle(&on_screen_pos, &PointFloat::new(10.0,10.0),
                                           graphics::Color::from_rgb(200,200,0), 170);
            }
        }
    }
    // Draw names.
    for (entity_id, owned, position) in CompIter2::<OwnedComp, PositionComp>::new(&ecs.c){
        let on_screen_pos = player_camera.game_space_to_screen_space(position.pos.clone());
        let player_name = ecs.c.get::<PlayerComp>(owned.owner).unwrap().name.clone();
        cool_batcher.add_text(on_screen_pos, player_name, graphics::Color::from((0,153,255)), 150);
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
                cool_batcher.add_rectangle_rect(graphics::Rect::new(screen_pos.x - 5.0, screen_pos.y - 5.0, 40.0,40.0),
                graphics::Color::from_rgb(255,204,0), 190);
            }
        }
        cool_batcher.add_rectangle(&screen_pos, &PointFloat::new(30.0,30.0),
                                   Color::from(ability_mould.button_info.color), 192);
        cool_batcher.add_text(screen_pos, hotkey.my_to_string(), Color::from_rgb(0,0,0), 193);
        cool_batcher.add_text(screen_pos.clone() + PointFloat::new(0.0,-30.0),
                              ability_mould.cost.to_string(), Color::from_rgb(0,0,200), 193);

    }
    // Draw resources.
    for (player_id, owns_resources) in CompIter1::<OwnsResourcesComp>::new(&ecs.c){
        if player_id == player_entity_id{
            for res_index in 0..RESOURCES_COUNT{
                let on_screen_pos = PointFloat::new(50.0 + res_index as f32 * 100.0, 50.0);

                let res_count = owns_resources.get_counti(res_index).to_string();
                cool_batcher.add_text(on_screen_pos, res_count, graphics::Color::from((255, 255, 255)), 200);
            }
        }
    }

    let mut test_param = DrawParam::new();
    // cool_batcher.add_image("factory.jpg".to_string(), test_param, 5);
    cool_batcher.gogo_draw(ctx, res);

    if rand::thread_rng().gen_bool(1.0){
        println!("{}", timer.stop_fmt());
    }
}


fn draw_text(ctx: &mut Context, on_screen_pos: PointFloat, text: String, color: graphics::Color) {
    let text_display = Text::new(text);

    graphics::draw(
        ctx,
        &text_display,
        (Point2::new(on_screen_pos.x, on_screen_pos.y), color),
    ).unwrap();
}

fn draw_rect(ctx: &mut Context, color: Color, mesh: graphics::Rect){
    let mode = graphics::DrawMode::fill();
    let mesh: graphics::Mesh = graphics::Mesh::new_rectangle(
        ctx,
        mode,
        mesh,
        color,
    ).unwrap();
    graphics::draw(
        ctx,
        &mesh,
        DrawParam::new(),
    ).unwrap();
}



trait MyToString{
    fn my_to_string(&self) -> String;
}
impl MyToString for VirtualKeyCode{
    fn my_to_string(&self) -> String {
        let test = format!("{:?}", self);
        return test;
    }
}