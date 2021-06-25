use ggez::{graphics, Context};
use ggez::graphics::{DrawParam, Text, Color, Mesh};
use crate::utils::gett;
use crate::rts::compsys::*;
use crate::ecs::{ActiveEcs, GlobalEntityID};
use crate::rts::game::game_state::UsingResources;
use nalgebra::Point2;

pub struct RenderComp{
    pub colour: (u8, u8, u8)
}

pub fn render(ecs: &mut ActiveEcs<UsingResources>, ctx: &mut Context, player_entity_id: GlobalEntityID){
    let player_camera = ecs.c.get::<CameraComp>(player_entity_id).unwrap();

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