use crate::*;
use becs::*;

pub fn new_cursor(player: GlobalEntityID, shade: Shade, z: u16) -> PendingEntity{
    PendingEntity::new5(
        CursorComp{
            player,
        },
        IgnoreHoverComp{
            useless: false
        },
        SizeComp{
            size: PointFloat::new(10.0,10.0),
        },
        PositionComp{
            pos: PointFloat::new(0.0,0.0),
        },
        RenderComp{
            z,
            texture: RenderTexture::Color(shade.0, shade.1, shade.2, 1.0),
            shape: RenderShape::Circle,
            only_render_owner: false
        }

    )
}