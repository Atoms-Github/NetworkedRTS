use std::collections::{BTreeSet};

use ggez::Context;
use ggez::graphics;
use ggez::graphics::DrawParam;
use serde::{Deserialize, Serialize};

use crate::create_system;
use crate::ecs::world::*;
use crate::gameplay::systems::position::PositionComp;
use crate::gameplay::systems::size::*;

create_system!( render_system | secret_render_system
	| my_position: PositionComp, my_render: RenderComp, my_size: SizeComp
	|
	| ctx:&mut Context
);

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct RenderComp {
    pub hue: (u8,u8,u8)//graphics::Color,
}

fn render_system(d: &mut Data, e: Entity, ctx: &mut Context) {
    let params = DrawParam::new();


    let arena_background : graphics::Mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(e.my_position(d).x, e.my_position(d).y,e.my_size(d).x,e.my_size(d).y),
        graphics::Color::from(e.my_render(d).hue),
//        graphics::Color::from_rgb(100,100,50),
    ).unwrap();




    graphics::draw(ctx, &arena_background, params).unwrap();
}










