use std::collections::{BTreeSet, BTreeMap};

use ggez::{Context, timer};
use ggez::graphics;
use ggez::graphics::{DrawParam, Text};
use serde::{Deserialize, Serialize};

use crate::ecs::rich_ecs::system_macro;
use crate::ecs::rich_ecs::world::*;
use crate::rts::systems::position::PositionComp;
use crate::rts::systems::player::PlayerComp;
use crate::rts::systems::size::*;
use crate::rts::systems::wasdmover::*;
use crate::pub_types::PlayerID;
use nalgebra::Point2;

create_system!( render_system | secret_render_system
	| my_position: PositionComp, my_render: RenderComp, my_size: SizeComp, my_wasdmover_comp: WasdMoverComp
	|
	| player_names: &BTreeMap<PlayerID, String>, ctx:&mut Context
);

#[derive(Debug,Serialize, Deserialize, Clone, Hash)]
pub struct RenderComp {
    pub hue: (u8,u8,u8)//graphics::Color,
}

fn render_system(d: &mut Data, e: Entity, player_names: &BTreeMap<PlayerID, String>, ctx: &mut Context) {
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


    let owner_id = e.my_wasdmover_comp(d).owner_id;

    let player_name = player_names.get(&owner_id).unwrap().clone();

    let fps = timer::fps(ctx);
    let player_name_display = Text::new(player_name);


    // When drawing through these calls, `DrawParam` will work as they are documented.
    graphics::draw(
        ctx,
        &player_name_display,
        (Point2::new(e.my_position(d).x, e.my_position(d).y), graphics::WHITE),
    ).unwrap();


}










