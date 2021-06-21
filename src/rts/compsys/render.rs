use ggez::{graphics, Context};
use ggez::graphics::{DrawParam, Text};
use crate::utils::gett;
use crate::rts::compsys::*;
use crate::ecs::ActiveEcs;
use crate::rts::game::game_state::UsingResources;
use nalgebra::Point2;

pub struct RenderComp{
    pub colour: (u8, u8, u8)
}

pub fn render(ecs: &mut ActiveEcs<UsingResources>, ctx: &mut Context){
    for entity in ecs.c.query(vec![crate::utils::gett::<RenderComp>(), crate::utils::gett::<PositionComp>()]){
        let position = ecs.c.get::<PositionComp>(entity).unwrap().clone();
        let render = ecs.c.get::<RenderComp>(entity).unwrap().clone();

        let mode = graphics::DrawMode::fill();
        let bounds = graphics::Rect::new(position.pos.x, position.pos.y,50.0, 50.0);
        let color = graphics::Color::from(render.colour);

        let arena_background : graphics::Mesh = graphics::Mesh::new_rectangle(
            ctx,
            mode,
            bounds,
            color,
        ).unwrap();


        graphics::draw(
            ctx,
            &arena_background,
            DrawParam::new(),
        ).unwrap();
    }
    for entity in ecs.c.query(vec![gett::<OwnedComp>(), gett::<PositionComp>()]){
        let position = ecs.c.get::<PositionComp>(entity).unwrap().clone();
        let owner = ecs.c.get::<OwnedComp>(entity).unwrap().owner;
        let player_name = ecs.c.get::<PlayerComp>(owner).unwrap().name.clone();

        let player_name = String::from_utf8(player_name.to_vec()).unwrap();

        let player_name_display = Text::new(player_name);

        graphics::draw(
            ctx,
            &player_name_display,
            (Point2::new(position.pos.x, position.pos.y), graphics::Color::from((0,153,255))),
        ).unwrap();
    }
}