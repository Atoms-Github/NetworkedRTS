use crate::ecs::pending_entity::PendingEntity;
use crate::rts::compsys::*;
use crate::ecs::GlobalEntityID;
use crate::pub_types::PointFloat;

impl PendingEntity{
    pub fn new_bullet(owner: GlobalEntityID, position: PointFloat) -> Self{
        Self::new6(
            RenderComp{ colour: (100,50,50) },
            PositionComp{ pos: position},
            CollisionComp{  },
            VelocityComp{ vel: PointFloat::new(1.0,1.0) },
            OwnedComp { owner },
            SizeComp{ size: PointFloat::new(50.0, 50.0) }
        )
    }
}