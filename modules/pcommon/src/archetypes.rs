use crate::*;
use becs::*;

pub fn new_cursor(player: GlobalEntityID, shade: Shade) -> PendingEntity{
    PendingEntity::new6(
        CursorComp{
            player,
        },
        UninteractableComp {
            useless: false
        },
        SizeComp{
            size: PointFloat::new(10.0,10.0),
        },
        PositionComp{
            pos: PointFloat::new(0.0,0.0),
        },
        RenderComp{
            z: CZValue::Cursor.g(),
            only_render_owner: false
        },
        SimpleViewerComp{
            texture: RenderTexture::Color(shade.0, shade.1, shade.2, 1.0),
            shape: RenderShape::Circle,
        }

    )
}