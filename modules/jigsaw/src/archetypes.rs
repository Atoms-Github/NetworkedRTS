use crate::*;
use becs::pending_entity::PendingEntity;


// pub fn new_bullet(owner: GlobalEntityID, position: PointFloat) -> PendingEntity{
//     PendingEntity::new6(
//         RenderComp{
//             z: 100,
//             texture: RenderTexture::Color(0.0,0.0,0.0,1.0),
//             shape: RenderShape::Circle,
//             only_render_owner: false
//         },
//         PositionComp{ pos: position},
//         CollisionComp{ useless: false },
//         VelocityComp{ vel: PointFloat::new(1.0,1.0) },
//         OwnedComp { owner },
//         SizeComp{ size: PointFloat::new(50.0, 50.0) },
//     )
// }