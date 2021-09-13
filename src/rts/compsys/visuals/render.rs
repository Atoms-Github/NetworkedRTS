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
use crate::ecs::comp_store::CompStorage;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RenderComp{
    pub z: u8,
    pub texture: RenderTexture,
    pub shape: RenderShape,
    pub only_render_owner: bool,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum RenderTexture{
    Color(f32, f32, f32, f32),
    Image(String)
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum RenderShape{
    Circle,
    Rectangle,
    Text(String)
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
                let small_top_left_game = PointFloat::new(x as f32 * arena_comp.get_box_length(),
                                                     y as f32 * arena_comp.get_box_length()) + &base_pos_game;
                let small_top_left_screen = player_camera.game_space_to_screen_space(small_top_left_game);
                let color = arena_comp.pathing[x][y].get_color().to_color();
                let rect = graphics::Rect::new(small_top_left_screen.x, small_top_left_screen.y, small_size.x, small_size.y);
                cool_batcher.add_rectangle_rect(rect, color, 50);
            }
        }
    }
    // Draw entities.
    for (entity_id, position, render, size) in
    CompIter3::<PositionComp, RenderComp, SizeComp>::new(&ecs.c){
        let (on_screen_pos, on_screen_size) = player_camera.get_as_screen_coords(&ecs.c, entity_id);
        match &render.shape{
            RenderShape::Circle => {
                let radius = ecs.c.get_unwrap::<SizeComp>(entity_id).size.x;
                match &render.texture{
                    RenderTexture::Color(r,g,b,a) => {
                        cool_batcher.add_circle(&on_screen_pos, radius, Color::new(*r,*g,*b,*a), render.z);
                    }
                    RenderTexture::Image(_) => {panic!("Render image circle isn't supported! (yet loh)")}
                }
            }
            RenderShape::Rectangle => {
                let size = &ecs.c.get_unwrap::<SizeComp>(entity_id).size;
                match &render.texture{
                    RenderTexture::Color(r,g,b,a) => {
                        cool_batcher.add_rectangle(&on_screen_pos, &on_screen_size,
                                                   Color::new(*r,*g,*b,*a), render.z);
                    }
                    RenderTexture::Image(image_name) => {
                        let mut params = DrawParam::new();
                        params.dest = mint::Point2::from([on_screen_pos.x, on_screen_pos.y]);
                        params.scale = mint::Vector2::from([size.x, size.y]);
                        cool_batcher.add_image(image_name.clone(), params, render.z);
                    }
                }
            }
            RenderShape::Text(text) => {}
        }
    }

    // Draw units.
    for (entity_id, position, render) in CompIter2::<PositionComp, RenderComp>::new(&ecs.c){
        let (on_screen_top_left, on_screen_size) = player_camera.get_as_screen_coords(&ecs.c, entity_id);

        if let Some(life_comp) = ecs.c.get::<LifeComp>(entity_id){
            let bar_centre = on_screen_top_left.clone() + PointFloat::new(on_screen_size.x / 2.0, -7.0);
            cool_batcher.add_progress_bar(&bar_centre, 5.0, life_comp.life,
                                          life_comp.max_life, Color::from_rgb(0,200,0),
                                          Color::from_rgb(255,0,0), 150);
        }
        if let Some(orders) = ecs.c.get::<OrdersComp>(entity_id){
            if let OrderState::CHANNELLING(channel_time) = &orders.state{
                let tech_tree = entity_id.get_owner_tech_tree(&ecs.c);
                let executing_order = orders.get_executing_order().unwrap();
                let ability = tech_tree.get_ability(executing_order.ability);

                let bar_centre = on_screen_top_left.clone() + PointFloat::new(on_screen_size.x / 2.0, -14.0);

                let max_width = 100.0;
                let current_width = *channel_time / ability.casting_time * max_width;

                cool_batcher.add_progress_bar(&bar_centre, 5.0, current_width,
                                              max_width, Color::from_rgb(52, 210, 235),
                                              Color::from_rgb(0,0,0), 150);
            }

        }

        if let Some(owned_comp) = ecs.c.get::<OwnedComp>(entity_id){
            let mut border_width = 3.0;
            let player_comp = ecs.c.get_unwrap::<PlayerComp>(owned_comp.owner);
            let mut border_color = player_comp.color.to_color();
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
            let border_top_left = on_screen_top_left.clone() - PointFloat::new(border_width, border_width);
            let border_size = on_screen_size.clone() + PointFloat::new(border_width * 2.0, border_width * 2.0);
            cool_batcher.add_rectangle(&border_top_left, &border_size,
                                       border_color, 100);
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
            cool_batcher.add_text(PointFloat::new(20.0, y), player.name.clone(), Color::from_rgb(0,0,0), 210);
            if player.alive{
                cool_batcher.add_text(PointFloat::new(150.0, y), "Alive".to_string(), Color::from_rgb(66, 245, 194), 210);
            }else{
                cool_batcher.add_text(PointFloat::new(150.0, y), "Ded".to_string(), Color::from_rgb(230,0,0), 210);
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
        // println!("{}", timer.stop_fmt());
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