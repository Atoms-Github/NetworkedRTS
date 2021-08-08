use ggez::{graphics, Context};
use ggez::graphics::{DrawParam, Text, Color, Mesh};
use crate::utils::gett;
use crate::rts::compsys::*;
use crate::ecs::{ActiveEcs, GlobalEntityID};
use crate::rts::game::game_state::UsingResources;
use nalgebra::Point2;
use crate::rts::compsys::owns_resources::{OwnsResourcesComp, RESOURCES_COUNT, ResourceType};

pub struct RenderComp{
    pub colour: (u8, u8, u8)
}

pub fn render(ecs: &mut ActiveEcs<UsingResources>, ctx: &mut Context, player_entity_id: GlobalEntityID){
    let player_camera = ecs.c.get::<CameraComp>(player_entity_id).unwrap();

    // Draw base.
    for (arena_id, arena_comp) in CompIter1::<ArenaComp>::new(&ecs.c){
        let screen_pos = player_camera.game_space_to_screen_space(arena_comp.get_top_left());
        let screen_size = player_camera.game_size_to_screen_size(arena_comp.get_size());

        draw_rect(ctx, graphics::Color::from((200,200,200)),
                  graphics::Rect::new(screen_pos.x, screen_pos.y, screen_size.x, screen_size.y));
    }

    for (arena_id, arena_comp) in CompIter1::<ArenaComp>::new(&ecs.c){
        let base_pos_game = arena_comp.get_top_left();
        let small_size =  player_camera.game_size_to_screen_size(
            PointFloat::new(arena_comp.get_box_length() as f32 - 1.0, arena_comp.get_box_length() as f32 - 1.0)
        );
        for x in 0..arena_comp.pathing.len(){
            for y in 0..arena_comp.pathing[x].len(){
                let small_top_left_game = PointFloat::new((x * arena_comp.get_box_length()) as f32,
                                                     (y * arena_comp.get_box_length()) as f32) + &base_pos_game;
                println!("SmallTopLEeft {:?}", small_top_left_game);
                let small_top_left_screen = player_camera.game_space_to_screen_space(small_top_left_game);
                draw_rect(ctx, graphics::Color::from((180,180,180)),
                          graphics::Rect::new(small_top_left_screen.x, small_top_left_screen.y, small_size.x, small_size.y));
            }
        }
    }

    for (entity_id, position, render) in CompIter2::<PositionComp, RenderComp>::new(&ecs.c){
        let (on_screen_pos, on_screen_size) = player_camera.get_as_screen_coords(&ecs.c, entity_id);

        draw_rect(ctx, graphics::Color::from(render.colour),
                  graphics::Rect::new(on_screen_pos.x, on_screen_pos.y, on_screen_size.x, on_screen_size.y));
        if let Some(life_comp) = ecs.c.get::<LifeComp>(entity_id){
            draw_rect(ctx, graphics::Color::from_rgb(200,0,0),
                      graphics::Rect::new(on_screen_pos.x, on_screen_pos.y,life_comp.max_life, 5.0));
            draw_rect(ctx, graphics::Color::from_rgb(0,200,0),
                      graphics::Rect::new(on_screen_pos.x, on_screen_pos.y,life_comp.life, 5.0));
        }
        if let Some(selectable_comp) = ecs.c.get::<SelectableComp>(entity_id){
            if selectable_comp.is_selected{
                draw_rect(ctx, graphics::Color::from_rgb(200,200,0),
                          graphics::Rect::new(on_screen_pos.x, on_screen_pos.y,10.0, 10.0));
            }


        }
    }
    for (entity_id, owned, position) in CompIter2::<OwnedComp, PositionComp>::new(&ecs.c){
        let on_screen_pos = player_camera.game_space_to_screen_space(position.pos.clone());
        let player_name = ecs.c.get::<PlayerComp>(owned.owner).unwrap().name.clone();
        let player_name = String::from_utf8(player_name.to_vec()).unwrap();
        let player_name_display = Text::new(player_name);

        graphics::draw(
            ctx,
            &player_name_display,
            (Point2::new(on_screen_pos.x, on_screen_pos.y), graphics::Color::from((0,153,255))),
        ).unwrap();
    }
    for (player_id, owns_resources) in CompIter1::<OwnsResourcesComp>::new(&ecs.c){
        if player_id == player_entity_id{
            for res_index in 0..RESOURCES_COUNT{
                let on_screen_pos = PointFloat::new(50.0 + res_index as f32 * 100.0, 50.0);

                let res_count = owns_resources.get_counti(res_index).to_string();
                let res_count_display = Text::new(res_count);

                graphics::draw(
                    ctx,
                    &res_count_display,
                    (Point2::new(on_screen_pos.x, on_screen_pos.y), graphics::Color::from((255,255,255))),
                ).unwrap();
            }
        }
    }

}
fn draw_rect(ctx: &mut Context, color: Color, mesh: graphics::Rect){

    let mode = graphics::DrawMode::fill();
    let bounds = mesh;
    let color = color;
    let mesh: graphics::Mesh = graphics::Mesh::new_rectangle(
        ctx,
        mode,
        bounds,
        color,
    ).unwrap();
    graphics::draw(
        ctx,
        &mesh,
        DrawParam::new(),
    ).unwrap();
}