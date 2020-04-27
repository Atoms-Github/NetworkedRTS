use std::collections::{BTreeSet};

use ggez::Context;
use ggez::graphics;
use ggez::graphics::DrawParam;
use serde::{Deserialize, Serialize};

use crate::create_system;
use crate::ecs::world::*;
use crate::gameplay::systems::position::PositionComp;
use crate::gameplay::systems::size::*;
use std::time::SystemTime;

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




    let mode = graphics::DrawMode::fill();
    let bounds = graphics::Rect::new(e.my_position(d).x, e.my_position(d).y,e.my_size(d).x,e.my_size(d).y);
    let color = graphics::Color::from(e.my_render(d).hue);

    let arena_background : graphics::Mesh = graphics::Mesh::new_rectangle(
        ctx,
        mode,
        bounds,
        color,
    ).unwrap();


    graphics::draw(ctx, &arena_background, params).unwrap();


}










